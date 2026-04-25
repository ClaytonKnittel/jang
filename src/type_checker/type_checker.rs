use cknittel_util::iter::CollectResult;

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
      type_expr::{InlineFn, TypeExpression, TypeExpressionVariant},
      unary_experssion::{UnaryExpression, UnaryOp},
      var::var_ref::VarRef,
    },
    token::literal::{Literal, NumericLiteral},
  },
  type_checker::{
    error::{TypeCheckerError, TypeCheckerResult},
    type_analysis::JangTypeAnalysis,
    type_inference_table::{InferredType, InferredTypeId, TypeConstraint, TypeInferenceTable},
    typed_ast_id::{TypedAstId, TypedAstIdTable},
    types::{concrete::ConcreteType, function::FunctionType, primitive::PrimitiveType},
  },
};

struct TypeChecker {
  types: TypeInferenceTable,
  ast_types: TypedAstIdTable<InferredTypeId>,
  current_fn: Option<AstGlobalDeclId>,
  bool_type_id: InferredTypeId,
}

impl TypeChecker {
  fn check(jang_file: &JangFile) -> TypeCheckerResult<JangTypeAnalysis> {
    let mut types = TypeInferenceTable::default();
    let bool_type_id = types.push(ConcreteType::Primitive(PrimitiveType::Bool));
    let mut checker = Self {
      types,
      ast_types: TypedAstIdTable::new(jang_file),
      current_fn: None,
      bool_type_id,
    };

    checker.register_global_types(jang_file)?;
    checker.check_jang_file(jang_file)?;

    let mut resolved_types = TypedAstIdTable::new(jang_file);
    for (ast_id, type_id) in checker.ast_types.into_iter() {
      resolved_types.insert(ast_id, checker.types.resolved(type_id));
    }

    Ok(JangTypeAnalysis::new(resolved_types))
  }

  fn add_type(&mut self, ty: ConcreteType) -> InferredTypeId {
    self.types.push(ty)
  }

  fn set_ast_type(&mut self, ast_id: impl Into<TypedAstId>, ty: ConcreteType) -> InferredTypeId {
    let type_id = self.types.push(ty);
    self.ast_types.insert(ast_id, type_id);
    type_id
  }

  fn set_ast_type_id(&mut self, ast_id: impl Into<TypedAstId>, type_id: InferredTypeId) {
    self.ast_types.insert(ast_id, type_id);
  }

  fn get_ast_type_id(&mut self, id: impl Into<TypedAstId>) -> InferredTypeId {
    *self
      .ast_types
      .get(id)
      .expect("Expected AST ID to have a populated type")
  }

  fn unify_types(
    &mut self,
    expected: InferredTypeId,
    actual: InferredTypeId,
  ) -> TypeCheckerResult<InferredTypeId> {
    self.types.unify(expected, actual)
  }

  fn unify_type_to_concrete(
    &mut self,
    expected: &ConcreteType,
    actual: InferredTypeId,
  ) -> TypeCheckerResult<InferredTypeId> {
    self.types.unify_to_concrete(expected, actual)
  }

  fn check_is_bool(&mut self, actual: InferredTypeId) -> TypeCheckerResult {
    self.unify_types(self.bool_type_id, actual)?;
    Ok(())
  }

  fn register_global_types(&mut self, jang_file: &JangFile) -> TypeCheckerResult {
    for fn_decl in jang_file.function_decls() {
      let fn_type = self.function_decl_type(fn_decl)?;
      self.set_ast_type(fn_decl.name_decl().id(), fn_type);
    }
    Ok(())
  }

  fn function_decl_type(&mut self, fn_decl: &FunctionDecl) -> TypeCheckerResult<ConcreteType> {
    let parameters = fn_decl
      .parameters()
      .iter()
      .map(|param| {
        let concrete = self.eval_type_expression(param.ty())?;
        self.set_ast_type(param.var().id(), concrete.clone());
        Ok(concrete)
      })
      .collect_result_vec()?;

    let return_type = Box::new(
      fn_decl
        .return_type()
        .map(|ty| self.eval_type_expression(ty))
        .unwrap_or(Ok(ConcreteType::Unit))?,
    );

    Ok(ConcreteType::Function(FunctionType::new(
      parameters,
      return_type,
    )))
  }

  fn check_jang_file(&mut self, jang_file: &JangFile) -> TypeCheckerResult {
    for fn_decl in jang_file.function_decls() {
      self.check_function_body(fn_decl)?;
    }
    Ok(())
  }

  fn check_function_body(&mut self, fn_decl: &FunctionDecl) -> TypeCheckerResult {
    let prev = self.current_fn.replace(fn_decl.name_decl().id());
    let result = self.check_block(fn_decl.body());
    self.current_fn = prev;

    result
  }

  fn check_statement(&mut self, stmt: &Statement) -> TypeCheckerResult {
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

  fn check_block(&mut self, block: &Block) -> TypeCheckerResult {
    for stmt in block.statements() {
      self.check_statement(stmt)?;
    }
    Ok(())
  }

  fn check_bind_statement(&mut self, s: &BindStatement) -> TypeCheckerResult {
    let expr_type_id = self.check_expression(s.expr())?;

    let Some(var_type_expr) = s.var_type() else {
      self.set_ast_type_id(s.var().id(), expr_type_id);
      return Ok(());
    };

    let var_type = self.eval_type_expression(var_type_expr)?;
    self.unify_type_to_concrete(&var_type, expr_type_id)?;
    Ok(())
  }

  fn check_rebind_statement(&mut self, s: &RebindStatement) -> TypeCheckerResult {
    let var_type_id = self.get_ast_type_id(s.var());
    let expr_type_id = self.check_expression(s.expr())?;
    self.unify_types(var_type_id, expr_type_id)?;
    Ok(())
  }

  fn check_ret_statement(&mut self, s: &RetStatement) -> TypeCheckerResult {
    let expr_type_id = self.check_expression(s.expr())?;

    let current_fn_type_id =
      self.get_ast_type_id(self.current_fn.expect("Unexpected ret outside a function"));

    let InferredType::Concrete(ConcreteType::Function(f)) = &self.types[current_fn_type_id] else {
      panic!("Expected current function to have FunctionType")
    };

    self.unify_type_to_concrete(&f.return_type().clone(), expr_type_id)?;
    Ok(())
  }

  fn check_if_statement(&mut self, s: &IfStatement) -> TypeCheckerResult {
    let cond_type_id = self.check_expression(s.condition())?;
    self.check_is_bool(cond_type_id)?;

    self.check_block(s.body())?;

    match s.else_clause() {
      ElseClause::None => Ok(()),
      ElseClause::Else(block) => self.check_block(block),
      ElseClause::ElseIf(nested) => self.check_if_statement(nested),
    }
  }

  fn check_expression(&mut self, expr: &Expression) -> TypeCheckerResult<InferredTypeId> {
    let type_id = match expr.variant() {
      ExpressionVariant::Literal(e) => self.check_literal_expression(e),
      ExpressionVariant::VarRef(e) => self.check_var_ref_expression(e),
      ExpressionVariant::BinaryExpression(e) => self.check_binary_expression(e)?,
      ExpressionVariant::UnaryExpression(e) => self.check_unary_expression(e)?,
      ExpressionVariant::CallExpression(e) => self.check_call_expression(e)?,
      ExpressionVariant::DotExpression(e) => self.check_dot_expression(e)?,
    };

    self.set_ast_type_id(expr.id(), type_id);
    Ok(type_id)
  }

  fn check_literal_expression(&mut self, expr: &LiteralExpression) -> InferredTypeId {
    match expr.literal() {
      Literal::Numeric(NumericLiteral::Integral(_)) => self.types.push(TypeConstraint::Integral),
      Literal::Numeric(NumericLiteral::Float(_)) => self.types.push(TypeConstraint::Floating),
    }
  }

  fn check_var_ref_expression(&mut self, var_ref: &VarRef) -> InferredTypeId {
    self.get_ast_type_id(var_ref)
  }

  fn check_binary_expression(
    &mut self,
    expr: &BinaryExpression,
  ) -> TypeCheckerResult<InferredTypeId> {
    let lhs = self.check_expression(expr.lhs())?;
    let rhs = self.check_expression(expr.rhs())?;
    let operand_type_id = self.unify_types(lhs, rhs)?;

    let invalid_err = |expected: &'static str| TypeCheckerError::InvalidOperand {
      op: expr.op(),
      expected: expected.to_owned(),
      actual: self.types[operand_type_id].clone(),
    };

    let operand_type = &self.types[operand_type_id];
    use BinaryOp::*;
    match expr.op() {
      Add | Sub | Mul | Div | Mod => operand_type
        .is_numeric()
        .then_some(operand_type_id)
        .ok_or_else(|| invalid_err("numeric")),
      Equal | NotEqual => (operand_type.is_integral() || operand_type.is_bool())
        .then_some(self.bool_type_id)
        .ok_or_else(|| invalid_err("integer or bool")),
      GreaterThan | GreaterThanEqual | LessThan | LessThanEqual => operand_type
        .is_numeric()
        .then_some(self.bool_type_id)
        .ok_or_else(|| invalid_err("numeric")),
      LogicalAnd | LogicalOr => operand_type
        .is_bool()
        .then_some(self.bool_type_id)
        .ok_or_else(|| invalid_err("bool")),
    }
  }

  fn check_unary_expression(
    &mut self,
    expr: &UnaryExpression,
  ) -> TypeCheckerResult<InferredTypeId> {
    let expr_type_id = self.check_expression(expr.expr())?;
    match expr.op() {
      UnaryOp::LogicalNot => {
        self.check_is_bool(expr_type_id)?;
        Ok(self.bool_type_id)
      }
    }
  }

  fn check_call_expression(&mut self, expr: &CallExpression) -> TypeCheckerResult<InferredTypeId> {
    let target_type_id = self.check_expression(expr.target())?;
    let InferredType::Concrete(ConcreteType::Function(f)) = self.types[target_type_id].clone()
    else {
      return Err(TypeCheckerError::NotCallable {
        target: self.types[target_type_id].clone(),
      });
    };

    // TODO: Remove this clone.
    let return_type_id = self.add_type(f.return_type().clone());

    let args = expr.argument_list();
    if args.len() != f.parameters().len() {
      return Err(TypeCheckerError::ArityMismatch {
        expected: f.parameters().len(),
        actual: args.len(),
      });
    }

    for (arg, param_type) in args.iter().zip(f.parameters()) {
      let arg_type = self.check_expression(arg)?;
      self.unify_type_to_concrete(param_type, arg_type)?;
    }

    Ok(return_type_id)
  }

  fn check_dot_expression(&mut self, _: &DotExpression) -> TypeCheckerResult<InferredTypeId> {
    todo!("Look up struct in global type decls")
  }

  fn eval_type_expression(
    &mut self,
    type_expression: &TypeExpression,
  ) -> TypeCheckerResult<ConcreteType> {
    match type_expression.variant() {
      TypeExpressionVariant::Unit => Ok(ConcreteType::Unit),
      TypeExpressionVariant::InlineFn(inline_fn) => self.eval_inline_fn(inline_fn),
      TypeExpressionVariant::Primitive(p) => Ok(ConcreteType::Primitive(p.into())),
      TypeExpressionVariant::Named(_) => todo!("Look up struct in global type decls"),
      TypeExpressionVariant::AnonymousStruct(_) => todo!("Handle structs"),
    }
  }

  fn eval_inline_fn(&mut self, inline_fn: &InlineFn) -> TypeCheckerResult<ConcreteType> {
    let return_type = Box::new(self.eval_type_expression(inline_fn.return_type())?);
    let parameters = inline_fn
      .args()
      .iter()
      .map(|arg| self.eval_type_expression(arg))
      .collect_result_vec()?;
    Ok(ConcreteType::Function(FunctionType::new(
      parameters,
      return_type,
    )))
  }
}

pub fn check(jang_file: &JangFile) -> TypeCheckerResult<JangTypeAnalysis> {
  TypeChecker::check(jang_file)
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{
    parser::{
      ast::{function_decl::FunctionDecl, jang_file::JangFile},
      grammar::testing::lex_and_parse_jang_file,
    },
    type_checker::{
      check,
      error::{
        TypeCheckerResult,
        matchers::{
          arity_mismatch_error, concrete_type_mismatch_error, invalid_operand, not_callable_error,
          type_mismatch_error,
        },
      },
      type_analysis::JangTypeAnalysis,
      type_inference_table::matchers::{concrete, inferred_floating, inferred_integral},
      types::{
        concrete::{ConcreteType, matchers::unit_type},
        function::matchers::{fn_param_types, fn_return_type},
        primitive::matchers::{bool_type, f32_type, f64_type, i32_type, i64_type},
      },
    },
  };

  fn type_check_file(source: &str) -> TypeCheckerResult<TypeCheckedFile> {
    let ast = lex_and_parse_jang_file(source.chars()).expect("parse should succeed");
    let analysis = check(&ast)?;
    Ok(TypeCheckedFile { ast, analysis })
  }

  fn type_check_ok(source: &str) -> TypeCheckedFile {
    type_check_file(source).unwrap()
  }

  struct TypeCheckedFile {
    ast: JangFile,
    analysis: JangTypeAnalysis,
  }

  // GoogleTest needs Debug, but annotating the AST with
  // types in a debug-friendly way is probably a fair bit of code.
  impl std::fmt::Debug for TypeCheckedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("TypeCheckedFile").finish()
    }
  }

  impl TypeCheckedFile {
    fn fn_decl_by_name(&self, name: &str) -> &FunctionDecl {
      self
        .ast
        .function_decls()
        .iter()
        .find(|f| f.name_decl().name().name() == name)
        .unwrap_or_else(|| panic!("function `{name}` not found"))
    }

    fn fn_type(&self, name: &str) -> &ConcreteType {
      self
        .analysis
        .get(self.fn_decl_by_name(name).name_decl().id())
    }
  }

  #[gtest]
  fn empty_fn() {
    let file = type_check_ok("fn foo() { }");
    expect_that!(
      file.fn_type("foo"),
      all![fn_param_types(is_empty()), fn_return_type(unit_type())]
    );
  }

  #[gtest]
  fn fn_with_return_value() {
    let file = type_check_ok("fn foo(): i32 { }");
    expect_that!(file.fn_type("foo"), fn_return_type(i32_type()));
  }

  #[gtest]
  fn fn_with_primitive_params() {
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
    );

    expect_that!(
      file.fn_type("foo"),
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
    let file = type_check_ok(
      r#"
        fn foo(
          p: (i32, f32) -> bool
        ) { }
        "#,
    );

    expect_that!(
      file.fn_type("foo"),
      fn_param_types(elements_are![all![
        fn_param_types(elements_are![i32_type(), f32_type()]),
        fn_return_type(bool_type()),
      ]])
    );
  }

  #[gtest]
  fn fn_call_with_arity_mismatch() {
    let file = type_check_file(
      r#"
        fn foo(x: i32) { }
        fn bar() { foo() }
        "#,
    );
    expect_that!(file, err(arity_mismatch_error(eq(&1), eq(&0))));
  }

  #[gtest]
  fn fn_call_of_non_callable_value() {
    let file = type_check_file(
      r#"
        fn foo(x: i32) { x() }
        "#,
    );
    expect_that!(file, err(not_callable_error(i32_type())));
  }

  #[gtest]
  fn fn_call_with_parameter_type_mismatch() {
    let file = type_check_file(
      r#"
        fn foo(x: i32) { }
        fn bar(x: f32) { foo(x) }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn fn_narrows_return_value() {
    expect_that!(type_check_file("fn foo(): i32 { ret 0 }"), ok(anything()));
  }

  #[gtest]
  fn fn_with_return_value_ok() {
    let file = type_check_file(
      r#"
        fn foo(x: i32): i32 { ret x }
        "#,
    );
    expect_that!(file, ok(anything()));
  }

  #[gtest]
  fn fn_return_value_type_mismatch() {
    let file = type_check_file(
      r#"
        fn foo(x: i64): i32 { ret x }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), i64_type()))
    );
  }

  #[gtest]
  fn unconstrained_integer_literal_is_i32() {
    let file = type_check_ok(
      r#"
        fn foo(): bool {
          ret 1 == 1
        }
        "#,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(file.analysis.get(bin_expr.lhs().id()), i32_type())
  }

  #[gtest]
  fn unconstrained_float_literal_is_f32() {
    let file = type_check_ok(
      r#"
        fn foo(): bool {
          ret 1. < 2.
        }
        "#,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(file.analysis.get(bin_expr.lhs().id()), f32_type())
  }

  #[gtest]
  fn bind_statement_without_type_ok() {
    type_check_ok(
      r#"
        fn foo(x: i32): i32 {
          let y = x
          ret y
        }
        "#,
    );
  }

  #[gtest]
  fn bind_statement_type_mismatch() {
    let f = type_check_file(
      r#"
        fn foo(x: i32): i32 {
          let y: f64 = x
        }
        "#,
    );
    expect_that!(f, err(concrete_type_mismatch_error(f64_type(), i32_type())))
  }

  #[gtest]
  fn bind_shadowing_name_ok() {
    type_check_ok(
      r#"
        fn foo(x: i32, y: f64) {
          let y = x
        }
        "#,
    );
  }

  #[gtest]
  fn rebind_type_mismatch() {
    expect_that!(
      type_check_file(
        r#"
        fn foo(x: i32, y: f64) {
          y = x
        }
        "#,
      ),
      err(concrete_type_mismatch_error(f64_type(), i32_type()))
    );
  }

  #[gtest]
  fn bind_variable_has_type_of_rhs() {
    let file = type_check_ok(
      r#"
        fn foo(x: i32) {
          let y = x
        }
        "#,
    );
    let bind_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_bind();
    expect_that!(file.analysis.get(bind_stmt.var().id()), i32_type())
  }

  #[gtest]
  fn call_expression_return_type_used() {
    let file = type_check_file(
      r#"
        fn foo(): f32 { ret 0. }
        fn bar() {
          let x: bool = foo()
        }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(bool_type(), f32_type()))
    );
  }

  #[gtest]
  fn comparison_requires_same_type() {
    type_check_ok("fn f(x: i32, y: i32): bool { ret x < y }");

    expect_that!(
      type_check_file("fn f(x: i32, y: f32): bool { ret x < y }"),
      err(concrete_type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn comparison_requires_numeric_type() {
    expect_that!(
      type_check_file("fn f(x: bool): bool { ret x < x }"),
      err(invalid_operand(contains_substring("numeric"), bool_type()))
    );
  }

  #[gtest]
  fn arithmetic_expression_requires_same_type() {
    type_check_ok("fn f(x: f32, y: f32) { let x = x - y }");

    expect_that!(
      type_check_file("fn f(x: f32, y: i32) { let x = x - y }"),
      err(concrete_type_mismatch_error(f32_type(), i32_type()))
    );
  }

  #[gtest]
  fn arithmetic_expression_preserves_operand_types() {
    type_check_ok("fn f(x: i32, y: i32): i32 { ret x - y }");

    expect_that!(
      type_check_file("fn f(x: i32, y: i32): f32 { ret x - y }"),
      err(concrete_type_mismatch_error(f32_type(), i32_type()))
    );
  }

  #[gtest]
  fn logical_and_requires_bools() {
    type_check_ok("fn f(): bool { ret 1 == 1 && 2 == 2 }");

    expect_that!(
      type_check_file("fn f(x: i32): bool { ret x && x }"),
      err(invalid_operand(contains_substring("bool"), i32_type()))
    );
  }

  #[gtest]
  fn logical_not_requires_bools() {
    type_check_ok("fn f(x: bool): bool { ret !x }");

    expect_that!(
      type_check_file("fn f(x: i32): bool { ret !x }"),
      err(concrete_type_mismatch_error(bool_type(), i32_type()))
    );
  }

  #[gtest]
  fn equality_requires_same_type() {
    type_check_ok("fn f(x: i32, y: i32): bool { ret x == y }");

    expect_that!(
      type_check_file("fn f(x: i32, y: f32): bool { ret x == y }"),
      err(concrete_type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn equality_not_defined_on_floats() {
    expect_that!(
      type_check_file("fn f(x: f64): bool { ret x == x }"),
      err(invalid_operand(
        contains_substring("integer or bool"),
        f64_type()
      ))
    );
  }

  #[gtest]
  fn if_condition_requires_bool() {
    type_check_ok("fn f(x: bool) { if x {} }");
    expect_that!(
      type_check_file("fn f(x: i32) { if x {} }",),
      err(concrete_type_mismatch_error(bool_type(), i32_type()))
    );
  }

  #[gtest]
  fn checks_inside_if_block() {
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
            x = y
          }
        }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn checks_inside_else_block() {
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
          } else {
            x = y
          }
        }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn checks_inside_if_else_block() {
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          if y {
          } else if y {
            x = y
          }
        }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn checks_inside_loop_block() {
    let file = type_check_file(
      r#"
        fn foo(x: i32, y: bool) {
          loop {
            x = y
          }
        }
        "#,
    );
    expect_that!(
      file,
      err(concrete_type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn function_type_in_local_binding_maintains_type() {
    type_check_ok(
      r#"
        fn f(): i32 { ret 0 }

        fn foo(): i32 {
          let func = f
          ret func()
        }
        "#,
    );
  }

  #[gtest]
  fn function_parameter_accepts_function_arg() {
    type_check_ok(
      r#"
        fn f(x: () -> i32): i32 { ret x() }
        fn g(): i32 { ret 0 }
        fn h(): i32 { ret f(g) }
        "#,
    );
  }

  #[gtest]
  fn function_parameter_rejects_incorrect_function_arg() {
    expect_that!(
      type_check_file(
        r#"
        fn f(x: () -> i32): i32 { ret x() }
        fn g(y: bool): i32 { ret 0 }
        fn h(): i32 { ret f(g) }
        "#,
      ),
      err(concrete_type_mismatch_error(
        fn_param_types(is_empty()),
        fn_param_types(elements_are![bool_type()]),
      ))
    )
  }

  #[gtest]
  fn cannot_perform_arithmetic_with_mismatched_literals() {
    expect_that!(
      type_check_file(
        r#"
        fn f() { let x = 1 - 1. }
        "#,
      ),
      err(type_mismatch_error(
        inferred_integral(),
        inferred_floating(),
      ))
    )
  }

  #[gtest]
  fn integral_literal_narrows_to_int_primitives() {
    type_check_ok("fn f(): i32 { ret 1 }");
    type_check_ok("fn f(): i64 { ret 1 }");
  }

  #[gtest]
  fn integral_literal_cannot_narrow_to_other_primitives() {
    expect_that!(
      type_check_file("fn f(): f32 { ret 1 }"),
      err(type_mismatch_error(
        concrete(f32_type()),
        inferred_integral(),
      ))
    );
    expect_that!(
      type_check_file("fn f(): f64 { ret 1 }"),
      err(type_mismatch_error(
        concrete(f64_type()),
        inferred_integral(),
      ))
    );
    expect_that!(
      type_check_file("fn f(): bool { ret 1 }"),
      err(type_mismatch_error(
        concrete(bool_type()),
        inferred_integral(),
      ))
    );
  }

  #[gtest]
  fn floating_literal_narrows_to_float_primitives() {
    type_check_ok("fn f(): f32 { ret 1. }");
    type_check_ok("fn f(): f64 { ret 1. }");
  }

  #[gtest]
  fn literal_in_ret_has_inferred_concrete_type() {
    let file = type_check_ok(
      r#"
        fn foo(): i64 {
          ret 1
        }
        "#,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    expect_that!(file.analysis.get(ret_stmt.expr().id()), i64_type());
  }

  #[gtest]
  fn literal_in_binary_expression_has_inferred_concrete_type() {
    let file = type_check_ok(
      r#"
        fn foo(): i64 {
          ret 1 + 1
        }
        "#,
    );
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(file.analysis.get(bin_expr.rhs().id()), i64_type());
  }

  #[gtest]
  fn literal_in_let_has_inferred_concrete_type() {
    let file = type_check_ok(
      r#"
        fn foo(): i64 {
          let x = 1
          ret x
        }
        "#,
    );
    let bind_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_bind();
    expect_that!(file.analysis.get(bind_stmt.expr().id()), i64_type());
  }

  #[gtest]
  fn unknown_literal_integer_defaults_to_i32() {
    let file = type_check_ok("fn foo(): bool { ret 1 < 2 }");
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(file.analysis.get(bin_expr.lhs().id()), i32_type())
  }

  #[gtest]
  fn unknown_literal_float_defaults_to_f32() {
    let file = type_check_ok("fn foo(): bool { ret 1. < 2. }");
    let ret_stmt = file.fn_decl_by_name("foo").body().statements()[0].as_ret();
    let bin_expr = ret_stmt.expr().variant().as_binary_expr();
    expect_that!(file.analysis.get(bin_expr.lhs().id()), f32_type())
  }
}
