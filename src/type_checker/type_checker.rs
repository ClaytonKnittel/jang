use std::ops::Deref;

use cknittel_util::{iter::CollectResult, result::EraseOk};

use crate::{
  parser::{
    ast::{
      binary_expression::{BinaryExpression, BinaryOp},
      bind_statement::BindStatement,
      block::Block,
      call_expression::CallExpression,
      dot_expression::DotExpression,
      expression::{Expression, ExpressionVariant},
      function_decl::FunctionDecl,
      id::def::AstGlobalDeclId,
      if_statement::{ElseClause, IfStatement},
      jang_file::JangFile,
      literal_expression::LiteralExpression,
      rebind_statement::RebindStatement,
      ret_statement::RetStatement,
      statement::Statement,
      structured_type_decl::{StructuredTypeDecl, StructuredTypeField},
      type_expr::{InlineFn, TypeExpression, TypeExpressionVariant},
      unary_experssion::{UnaryExpression, UnaryOp},
      var::var_ref::VarRef,
    },
    token::literal::{Literal, NumericLiteral},
  },
  type_checker::{
    context::TypeCheckerCtx,
    error::{TypeCheckerError, TypeCheckerResult},
    inference::{InferenceTable, InferredTy, TypeClass},
    type_analysis::JangTypeAnalysis,
    typed_ast_id::{TypedAstId, TypedAstIdTable},
    types::{
      concrete::ConcreteType,
      primitive::PrimitiveType,
      registry::{Ty, TypeRegistry},
      strukt::StructField,
    },
  },
};

struct TypeChecker<'ctx> {
  types: TypeRegistry<'ctx>,
  ast_types: TypedAstIdTable<InferredTy<'ctx>>,
  inference: InferenceTable<'ctx>,
  current_fn: Option<AstGlobalDeclId>,
}

impl<'ctx> TypeChecker<'ctx> {
  fn check(
    jang_file: &JangFile,
    ctx: &'ctx TypeCheckerCtx<'ctx>,
  ) -> TypeCheckerResult<'ctx, JangTypeAnalysis<'ctx>> {
    let mut checker = Self {
      types: TypeRegistry::new(ctx),
      ast_types: TypedAstIdTable::new(jang_file),
      inference: InferenceTable::default(),
      current_fn: None,
    };

    checker.register_global_types(jang_file)?;
    checker.check_jang_file(jang_file)?;

    let mut resolved_types = TypedAstIdTable::new(jang_file);
    for (ast_id, ty) in checker.ast_types.into_iter() {
      resolved_types.insert(ast_id, checker.inference.resolve(ty, &checker.types));
    }

    Ok(JangTypeAnalysis::new(resolved_types))
  }

  fn set_ast_type(&mut self, ast_id: impl Into<TypedAstId>, ty: InferredTy<'ctx>) {
    self.ast_types.insert(ast_id, ty);
  }

  fn get_ast_type(&self, id: impl Into<TypedAstId>) -> InferredTy<'ctx> {
    *self
      .ast_types
      .get(id)
      .expect("Expected AST ID to have a populated type")
  }

  fn unify(
    &mut self,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    self.inference.unify(expected, actual, &self.types)
  }

  fn check_is_bool(&mut self, actual: InferredTy<'ctx>) -> TypeCheckerResult<'ctx> {
    self
      .unify(
        self.types.primitive_type(PrimitiveType::Bool).into(),
        actual,
      )
      .erase_ok()
  }

  fn register_global_types(&mut self, jang_file: &JangFile) -> TypeCheckerResult<'ctx> {
    for fn_decl in jang_file.function_decls() {
      let fn_type = self.function_decl_type(fn_decl)?;
      self.set_ast_type(fn_decl.name_decl().id(), fn_type.into());
    }
    Ok(())
  }

  fn function_decl_type(&mut self, fn_decl: &FunctionDecl) -> TypeCheckerResult<'ctx, Ty<'ctx>> {
    let parameters = fn_decl
      .parameters()
      .iter()
      .map(|param| {
        let ty = self.eval_type_expression(param.ty())?;
        self.set_ast_type(param.var().id(), ty.into());
        Ok(ty)
      })
      .collect_result_vec()?;

    let return_type = fn_decl
      .return_type()
      .map(|ty| self.eval_type_expression(ty))
      .unwrap_or(Ok(self.types.unit_type()))?;

    Ok(self.types.function_type(parameters, return_type))
  }

  fn check_jang_file(&mut self, jang_file: &JangFile) -> TypeCheckerResult<'ctx> {
    for fn_decl in jang_file.function_decls() {
      self.check_function_body(fn_decl)?;
    }
    Ok(())
  }

  fn check_function_body(&mut self, fn_decl: &FunctionDecl) -> TypeCheckerResult<'ctx> {
    let prev = self.current_fn.replace(fn_decl.name_decl().id());
    let result = self.check_block(fn_decl.body());
    self.current_fn = prev;

    result
  }

  fn check_statement(&mut self, stmt: &Statement) -> TypeCheckerResult<'ctx> {
    match stmt {
      Statement::Bind(s) => self.check_bind_statement(s),
      Statement::Rebind(s) => self.check_rebind_statement(s),
      Statement::Ret(s) => self.check_ret_statement(s),
      Statement::CallStatement(s) => self.check_call_expression(s).map(drop),
      Statement::IfStatement(s) => self.check_if_statement(s),
      Statement::LoopStatement(s) => self.check_block(s.body()),
      Statement::Block(s) => self.check_block(s),
      Statement::Break => Ok(()),
    }
  }

  fn check_block(&mut self, block: &Block) -> TypeCheckerResult<'ctx> {
    for stmt in block.statements() {
      self.check_statement(stmt)?;
    }
    Ok(())
  }

  fn check_bind_statement(&mut self, s: &BindStatement) -> TypeCheckerResult<'ctx> {
    let expr_type = self.check_expression(s.expr())?;

    let var_type = s
      .var_type()
      .map(|type_expr| -> TypeCheckerResult<'ctx, _> {
        let var_type = self.eval_type_expression(type_expr)?;
        self.unify(var_type.into(), expr_type)
      })
      .transpose()?
      .unwrap_or(expr_type);

    self.set_ast_type(s.var().id(), var_type);
    Ok(())
  }

  fn check_rebind_statement(&mut self, s: &RebindStatement) -> TypeCheckerResult<'ctx> {
    let var_type = self.get_ast_type(s.var());
    let expr_type = self.check_expression(s.expr())?;
    self.unify(var_type, expr_type).erase_ok()
  }

  fn check_ret_statement(&mut self, s: &RetStatement) -> TypeCheckerResult<'ctx> {
    let expr_type = self.check_expression(s.expr())?;

    let current_fn_type = self.inference.resolve(
      self.get_ast_type(self.current_fn.expect("Unexpected ret outside a function")),
      &self.types,
    );

    let ConcreteType::Function(f) = current_fn_type.deref() else {
      panic!("Expected current function to have FunctionType")
    };
    let return_type = f.return_type();

    self.unify(return_type.into(), expr_type).erase_ok()
  }

  fn check_if_statement(&mut self, s: &IfStatement) -> TypeCheckerResult<'ctx> {
    let cond_type = self.check_expression(s.condition())?;
    self.check_is_bool(cond_type)?;

    self.check_block(s.body())?;

    match s.else_clause() {
      ElseClause::None => Ok(()),
      ElseClause::Else(block) => self.check_block(block),
      ElseClause::ElseIf(nested) => self.check_if_statement(nested),
    }
  }

  fn check_expression(&mut self, expr: &Expression) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    let ty = match expr.variant() {
      ExpressionVariant::Literal(e) => self.check_literal_expression(e),
      ExpressionVariant::VarRef(e) => self.check_var_ref_expression(e),
      ExpressionVariant::BinaryExpression(e) => self.check_binary_expression(e)?,
      ExpressionVariant::UnaryExpression(e) => self.check_unary_expression(e)?,
      ExpressionVariant::CallExpression(e) => self.check_call_expression(e)?,
      ExpressionVariant::DotExpression(e) => self.check_dot_expression(e)?,
    };

    self.set_ast_type(expr.id(), ty);
    Ok(ty)
  }

  fn check_literal_expression(&mut self, expr: &LiteralExpression) -> InferredTy<'ctx> {
    match expr.literal() {
      Literal::Numeric(NumericLiteral::Integral(_)) => self.inference.new_integral_var(),
      Literal::Numeric(NumericLiteral::Float(_)) => self.inference.new_floating_var(),
    }
  }

  fn check_var_ref_expression(&mut self, var_ref: &VarRef) -> InferredTy<'ctx> {
    self.get_ast_type(var_ref)
  }

  fn check_binary_expression(
    &mut self,
    expr: &BinaryExpression,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    let lhs = self.check_expression(expr.lhs())?;
    let rhs = self.check_expression(expr.rhs())?;
    let operand_ty = self.unify(lhs, rhs)?;

    let bool_ty: InferredTy<'ctx> = self.types.primitive_type(PrimitiveType::Bool).into();

    use BinaryOp::*;
    match expr.op() {
      Add | Sub | Mul | Div | Mod => {
        self
          .inference
          .check_requirement(operand_ty, TypeClass::Numeric, &self.types)?;
        Ok(operand_ty)
      }
      Equal | NotEqual => {
        self
          .inference
          .check_requirement(operand_ty, TypeClass::Eq, &self.types)?;
        Ok(bool_ty)
      }
      GreaterThan | GreaterThanEqual | LessThan | LessThanEqual => {
        self
          .inference
          .check_requirement(operand_ty, TypeClass::Numeric, &self.types)?;
        Ok(bool_ty)
      }
      LogicalAnd | LogicalOr => {
        self.check_is_bool(operand_ty)?;
        Ok(bool_ty)
      }
    }
  }

  fn check_unary_expression(
    &mut self,
    expr: &UnaryExpression,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    let expr_type = self.check_expression(expr.expr())?;
    match expr.op() {
      UnaryOp::LogicalNot => {
        self.check_is_bool(expr_type)?;
        Ok(self.types.primitive_type(PrimitiveType::Bool).into())
      }
    }
  }

  fn check_call_expression(
    &mut self,
    expr: &CallExpression,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    let target_type = self.check_expression(expr.target())?;
    let target_type = self.inference.resolve(target_type, &self.types);
    let ConcreteType::Function(f) = target_type.deref() else {
      return Err(TypeCheckerError::NotCallable {
        target: target_type,
      });
    };

    let return_type = f.return_type();

    let args = expr.argument_list();
    if args.len() != f.parameters().len() {
      return Err(TypeCheckerError::ArityMismatch {
        expected: f.parameters().len(),
        actual: args.len(),
      });
    }

    for (arg, &param_type) in args.iter().zip(f.parameters()) {
      let arg_type = self.check_expression(arg)?;
      self.unify(param_type.into(), arg_type)?;
    }

    Ok(return_type.into())
  }

  fn check_dot_expression(
    &mut self,
    dot_expr: &DotExpression,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    let base_inferred_ty = self.check_expression(dot_expr.base())?;
    let base_ty = self.inference.resolve(base_inferred_ty, &self.types);
    let invalid_member_access = || TypeCheckerError::InvalidMemberAccess {
      target: base_ty,
      member: dot_expr.member().clone(),
    };

    let ConcreteType::Struct(s) = base_ty.deref() else {
      return Err(invalid_member_access());
    };
    let Some(member_ty) = s.field_ty(dot_expr.member()) else {
      return Err(invalid_member_access());
    };

    Ok(member_ty.into())
  }

  fn eval_type_expression(
    &mut self,
    type_expression: &TypeExpression,
  ) -> TypeCheckerResult<'ctx, Ty<'ctx>> {
    match type_expression.variant() {
      TypeExpressionVariant::Unit => Ok(self.types.unit_type()),
      TypeExpressionVariant::InlineFn(inline_fn) => self.eval_inline_fn(inline_fn),
      TypeExpressionVariant::Primitive(p) => Ok(self.types.primitive_type(p.into())),
      TypeExpressionVariant::Named(_) => todo!("Look up struct in global type decls"),
      TypeExpressionVariant::AnonymousStruct(s) => self.eval_struct_type(s),
    }
  }

  fn eval_inline_fn(&mut self, inline_fn: &InlineFn) -> TypeCheckerResult<'ctx, Ty<'ctx>> {
    let return_type = self.eval_type_expression(inline_fn.return_type())?;
    let parameters = inline_fn
      .args()
      .iter()
      .map(|arg| self.eval_type_expression(arg))
      .collect_result_vec()?;
    Ok(self.types.function_type(parameters, return_type))
  }

  fn eval_struct_type(
    &mut self,
    struct_type: &StructuredTypeDecl,
  ) -> TypeCheckerResult<'ctx, Ty<'ctx>> {
    let fields = struct_type
      .fields()
      .iter()
      .map(|f| self.eval_struct_field_type(f))
      .collect_result_vec()?;
    Ok(self.types.struct_type(fields))
  }

  fn eval_struct_field_type(
    &mut self,
    field: &StructuredTypeField,
  ) -> TypeCheckerResult<'ctx, StructField<'ctx>> {
    Ok(StructField::new(
      field.name().clone(),
      self.eval_type_expression(field.ty())?,
    ))
  }
}

pub fn check<'ctx>(
  jang_file: &JangFile,
  ctx: &'ctx TypeCheckerCtx<'ctx>,
) -> TypeCheckerResult<'ctx, JangTypeAnalysis<'ctx>> {
  TypeChecker::check(jang_file, ctx)
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    parser::{
      ast::{function_decl::FunctionDecl, jang_file::JangFile},
      grammar::testing::lex_and_parse_jang_file,
      token::ident::matchers::ident,
    },
    type_checker::{
      check,
      context::TypeCheckerCtx,
      error::{
        TypeCheckerResult,
        matchers::{
          arity_mismatch_error, invalid_member_access, not_callable_error, type_class_mismatch,
          type_mismatch_error,
        },
      },
      inference::TypeClass,
      type_analysis::JangTypeAnalysis,
      types::{
        concrete::matchers::unit_type,
        function::matchers::{fn_param_types, fn_return_type},
        primitive::matchers::{bool_type, f32_type, f64_type, i32_type, i64_type},
        registry::Ty,
        strukt::matchers::{struct_field, struct_fields},
      },
    },
  };

  fn type_check_file<'ctx>(
    source: &str,
    ctx: &'ctx TypeCheckerCtx<'ctx>,
  ) -> TypeCheckerResult<'ctx, TypeCheckedFile<'ctx>> {
    let ast = lex_and_parse_jang_file(source.chars()).expect("parse should succeed");
    let analysis = check(&ast, ctx)?;
    Ok(TypeCheckedFile { ast, analysis })
  }

  fn type_check_ok<'ctx>(source: &str, ctx: &'ctx TypeCheckerCtx<'ctx>) -> TypeCheckedFile<'ctx> {
    type_check_file(source, ctx).unwrap()
  }

  struct TypeCheckedFile<'ctx> {
    ast: JangFile,
    analysis: JangTypeAnalysis<'ctx>,
  }

  // GoogleTest needs Debug, but annotating the AST with
  // types in a debug-friendly way is probably a fair bit of code.
  impl<'ctx> std::fmt::Debug for TypeCheckedFile<'ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("TypeCheckedFile").finish()
    }
  }

  impl<'ctx> TypeCheckedFile<'ctx> {
    fn fn_decl_by_name(&self, name: &str) -> &FunctionDecl {
      self
        .ast
        .function_decls()
        .iter()
        .find(|f| f.name_decl().name().name() == name)
        .unwrap_or_else(|| panic!("function `{name}` not found"))
    }

    fn fn_type(&self, name: &str) -> Ty<'ctx> {
      self
        .analysis
        .get(self.fn_decl_by_name(name).name_decl().id())
    }
  }

  #[gtest]
  fn empty_fn() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo() { }", &ctx);
    expect_that!(
      &file.fn_type("foo"),
      all![fn_param_types(is_empty()), fn_return_type(unit_type())],
    );
  }

  #[gtest]
  fn fn_with_return_value() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): i32 { }", &ctx);
    expect_that!(&file.fn_type("foo"), fn_return_type(i32_type()));
  }

  #[gtest]
  fn fn_with_primitive_params() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn foo(
          p0: i32,
          p1: i64,
          p2: f32,
          p3: f64,
          p4: bool
        ) { }
        "#,
      &ctx,
    );

    expect_that!(
      &file.fn_type("foo"),
      fn_param_types(elements_are![
        i32_type(),
        i64_type(),
        f32_type(),
        f64_type(),
        bool_type()
      ])
    );
  }

  #[gtest]
  fn fn_with_inline_param() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn foo(
          p: (i32, f32) -> bool
        ) { }
        "#,
      &ctx,
    );

    expect_that!(
      &file.fn_type("foo"),
      fn_param_types(elements_are![all![
        fn_param_types(elements_are![i32_type(), f32_type()]),
        fn_return_type(bool_type()),
      ]])
    );
  }

  #[gtest]
  fn fn_call_with_arity_mismatch() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32) { }
        fn bar() { foo() }
        "#,
      &ctx,
    );
    expect_that!(file, err(arity_mismatch_error(eq(&1), eq(&0))));
  }

  #[gtest]
  fn fn_call_of_non_callable_value() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32) { x() }
        "#,
      &ctx,
    );
    expect_that!(file, err(not_callable_error(i32_type())));
  }

  #[gtest]
  fn fn_call_with_parameter_type_mismatch() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32) { }
        fn bar(x: f32) { foo(x) }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), f32_type())));
  }

  #[gtest]
  fn fn_with_return_value_ok() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32): i32 { ret x }
        "#,
      &ctx,
    );
    expect_that!(file, ok(anything()));
  }

  #[gtest]
  fn fn_return_value_type_mismatch() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i64): i32 { ret x }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), i64_type())));
  }

  #[gtest]
  fn unconstrained_integer_literal_is_i32() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn foo(): bool {
          ret 1 == 1
        }
        "#,
      &ctx,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(&file.analysis.get(bin_expr.lhs().id()), i32_type())
  }

  #[gtest]
  fn unconstrained_float_literal_is_f32() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn foo(): bool {
          ret 1. < 2.
        }
        "#,
      &ctx,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(&file.analysis.get(bin_expr.lhs().id()), f32_type())
  }

  #[gtest]
  fn integer_literal_infers_i64_from_return_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): i64 { ret 1 }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    expect_that!(&file.analysis.get(ret_stmt.expr().id()), i64_type())
  }

  #[gtest]
  fn integer_literal_infers_i32_from_return_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): i32 { ret 1 }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    expect_that!(&file.analysis.get(ret_stmt.expr().id()), i32_type())
  }

  #[gtest]
  fn float_literal_infers_i64_from_return_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): f64 { ret 1. }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    expect_that!(&file.analysis.get(ret_stmt.expr().id()), f64_type())
  }

  #[gtest]
  fn float_literal_infers_f32_from_return_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): f32 { ret 1. }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    expect_that!(&file.analysis.get(ret_stmt.expr().id()), f32_type())
  }

  #[gtest]
  fn typed_bind_infers_numeric_literal_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo() { let x: i64 = 1 }", &ctx);
    let bind_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_bind();
    expect_that!(&file.analysis.get(bind_stmt.var().id()), i64_type());
    expect_that!(&file.analysis.get(bind_stmt.expr().id()), i64_type())
  }

  #[gtest]
  fn arithmetic_literals_infer_from_expected_result_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(): i64 { ret 1 + 2 }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(&file.analysis.get(ret_stmt.expr().id()), i64_type());
    expect_that!(&file.analysis.get(bin_expr.lhs().id()), i64_type());
    expect_that!(&file.analysis.get(bin_expr.rhs().id()), i64_type())
  }

  #[gtest]
  fn numeric_literal_infers_from_other_binary_operand() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok("fn foo(x: i64): i64 { ret 1 + x }", &ctx);
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(&file.analysis.get(bin_expr.lhs().id()), i64_type())
  }

  #[gtest]
  fn function_argument_infers_numeric_literal_type() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn takes_i64(x: i64) {}
        fn foo() { takes_i64(1) }
        "#,
      &ctx,
    );

    let call = file.fn_decl_by_name("foo").body().statements()[0].as_call();
    expect_that!(&file.analysis.get(call.argument_list()[0].id()), i64_type())
  }

  #[gtest]
  fn integer_literal_does_not_infer_float_type() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file("fn foo(): f64 { ret 1 }", &ctx),
      err(type_mismatch_error(f64_type(), i32_type()))
    )
  }

  #[gtest]
  fn integral_comparison() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn foo(): bool { ret 1 == 1 }", &ctx);
  }

  #[gtest]
  fn delayed_inference_through_literal_comparison() {
    let ctx = TypeCheckerCtx::default();
    let f = type_check_ok(
      "fn foo(): bool {
                let x = 1
                let y: i64 = 3
                ret x == 2 && x == y
              }",
      &ctx,
    );
    let two_expr = f.fn_decl_by_name("foo").body().statements()[2]
      .as_ret()
      .expr()
      .variant()
      .as_binary_expr()
      .lhs()
      .variant()
      .as_binary_expr()
      .rhs();
    expect_that!(&f.analysis.get(two_expr.id()), i64_type())
  }

  #[gtest]
  fn previously_inferred_type_can_cause_mismatch() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file(
        "fn foo(x: i64) {
                let x = 1
                let y: i64 = x
                let z: i32 = x
              }",
        &ctx,
      ),
      err(type_mismatch_error(i32_type(), i64_type()))
    )
  }

  #[gtest]
  fn bind_statement_without_type_ok() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn foo(x: i32): i32 {
          let y = x
          ret y
        }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn bind_statement_type_mismatch() {
    let ctx = TypeCheckerCtx::default();
    let f = type_check_file(
      r#"
        fn foo(x: i32): i32 {
          let y: f64 = x
        }
        "#,
      &ctx,
    );
    expect_that!(f, err(type_mismatch_error(f64_type(), i32_type())))
  }

  #[gtest]
  fn bind_shadowing_name_ok() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn foo(x: i32, y: f64) {
          let y = x
        }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn rebind_type_mismatch() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file(
        r#"
        fn foo(x: i32, y: f64) {
          y = x
        }
        "#,
        &ctx
      ),
      err(type_mismatch_error(f64_type(), i32_type()))
    );
  }

  #[gtest]
  fn bind_variable_has_type_of_rhs() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn foo(x: i32) {
          let y = x
        }
        "#,
      &ctx,
    );
    let bind_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_bind();
    expect_that!(&file.analysis.get(bind_stmt.var().id()), i32_type())
  }

  #[gtest]
  fn call_expression_return_type_used() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(): f32 { ret 0. }
        fn bar() {
          let x: bool = foo()
        }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(bool_type(), f32_type())));
  }

  #[gtest]
  fn comparison_requires_same_type() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: i32, y: i32): bool { ret x < y }", &ctx);

    expect_that!(
      type_check_file("fn f(x: i32, y: f32): bool { ret x < y }", &ctx),
      err(type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn comparison_requires_numeric_type() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file("fn f(x: bool): bool { ret x < x }", &ctx),
      err(type_class_mismatch(pat!(TypeClass::Numeric), bool_type()))
    );
  }

  #[gtest]
  fn arithmetic_expression_requires_same_type() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: f32, y: f32) { let x = x - y }", &ctx);

    expect_that!(
      type_check_file("fn f(x: f32, y: i32) { let x = x - y }", &ctx),
      err(type_mismatch_error(f32_type(), i32_type()))
    );
  }

  #[gtest]
  fn arithmetic_expression_preserves_operand_types() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: i32, y: i32): i32 { ret x - y }", &ctx);

    expect_that!(
      type_check_file("fn f(x: i32, y: i32): f32 { ret x - y }", &ctx),
      err(type_mismatch_error(f32_type(), i32_type()))
    );
  }

  #[gtest]
  fn logical_and_requires_bools() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(): bool { ret 1 == 1 && 2 == 2 }", &ctx);

    expect_that!(
      type_check_file("fn f(x: i32): bool { ret x && x }", &ctx),
      err(type_mismatch_error(bool_type(), i32_type()))
    );
  }

  #[gtest]
  fn logical_not_requires_bools() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: bool): bool { ret !x }", &ctx);

    expect_that!(
      type_check_file("fn f(x: i32): bool { ret !x }", &ctx),
      err(type_mismatch_error(bool_type(), i32_type()))
    );
  }

  #[gtest]
  fn equality_requires_same_type() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: i32, y: i32): bool { ret x == y }", &ctx);

    expect_that!(
      type_check_file("fn f(x: i32, y: f32): bool { ret x == y }", &ctx),
      err(type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn equality_not_defined_on_floats() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file("fn f(x: f64): bool { ret x == x }", &ctx),
      err(type_class_mismatch(pat!(TypeClass::Eq), f64_type()))
    );
  }

  #[gtest]
  fn if_condition_requires_bool() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok("fn f(x: bool) { if x {} }", &ctx);
    expect_that!(
      type_check_file("fn f(x: i32) { if x {} }", &ctx),
      err(type_mismatch_error(bool_type(), i32_type()))
    );
  }

  #[gtest]
  fn checks_inside_if_block() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
            x = y
          }
        }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), bool_type())));
  }

  #[gtest]
  fn checks_inside_else_block() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
          } else {
            x = y
          }
        }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), bool_type())));
  }

  #[gtest]
  fn checks_inside_if_else_block() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
          } else if y {
            x = y
          }
        }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), bool_type())));
  }

  #[gtest]
  fn checks_inside_loop_block() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          loop {
            x = y
          }
        }
        "#,
      &ctx,
    );
    expect_that!(file, err(type_mismatch_error(i32_type(), bool_type())));
  }

  #[gtest]
  fn function_type_in_local_binding_maintains_type() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn f(): i32 { ret 0 }

        fn foo(): i32 {
          let func = f
          ret func()
        }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn function_parameter_accepts_function_arg() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn f(x: () -> i32): i32 { ret x() }
        fn g(): i32 { ret 0 }
        fn h(): i32 { ret f(g) }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn function_parameter_rejects_incorrect_function_arg() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file(
        r#"
        fn f(x: () -> i32): i32 { ret x() }
        fn g(y: bool): i32 { ret 0 }
        fn h(): i32 { ret f(g) }
        "#,
        &ctx
      ),
      err(type_mismatch_error(
        fn_param_types(is_empty()),
        fn_param_types(elements_are![bool_type()]),
      ))
    )
  }

  #[gtest]
  fn struct_member_access() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn f(x: { y: i32 }): i32 {
          ret x.y
        }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn struct_member_fn_call() {
    let ctx = TypeCheckerCtx::default();
    type_check_ok(
      r#"
        fn f(a: { x: () -> i64 }): i64 {
          ret a.x()
        }
        "#,
      &ctx,
    );
  }

  #[gtest]
  fn struct_member_invalid_base() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file(
        r#"
        fn f(a: i32): i64 {
          ret a.x
        }
        "#,
        &ctx,
      ),
      err(invalid_member_access(i32_type(), ident("x")))
    );
  }

  #[gtest]
  fn struct_member_invalid_name_in_member() {
    let ctx = TypeCheckerCtx::default();
    expect_that!(
      type_check_file(
        r#"
        fn f(a: i32): i64 {
          ret a.does_not_exist
        }
        "#,
        &ctx,
      ),
      err(invalid_member_access(i32_type(), ident("does_not_exist")))
    );
  }

  #[gtest]
  fn struct_type_in_expr() {
    let ctx = TypeCheckerCtx::default();
    let file = type_check_ok(
      r#"
        fn f(a: {f0: i64 f1: i32}): i64 {
          ret a.f0
        }
        "#,
      &ctx,
    );

    let ret_stmt = file.fn_decl_by_name("f").body().statements()[0].as_ret();
    let dot_expr = ret_stmt.expr().variant().as_dot_expr();
    expect_that!(
      &file.analysis.get(dot_expr.base().id()),
      struct_fields(elements_are![
        struct_field(ident("f0"), i64_type()),
        struct_field(ident("f1"), i32_type())
      ])
    )
  }
}
