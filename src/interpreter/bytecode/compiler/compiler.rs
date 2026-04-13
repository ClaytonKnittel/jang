use cknittel_util::from_variants::FromVariants;

use crate::{
  interpreter::{
    bytecode::compiler::{
      instruction::{
        ConditionalJumpTargets, JitCallInstruction, JitCompiledFunction, JitInstruction,
        JitInstructionBlock, JitTerminalInstruction,
      },
      instruction_block_list::{BlockId, BlockList, BlockListBuilder},
      lexical_scope::JitCompilerLexicalScope,
      loop_context::LoopContext,
    },
    error::{InterpreterError, InterpreterResult},
  },
  parser::{
    ast::{
      binary_expression::BinaryExpression,
      bind_statement::{BindStatement, Mutability},
      block::Block,
      call_expression::CallExpression,
      expression::{Expression, ExpressionVariant},
      function_decl::FunctionDecl,
      if_statement::{ElseClause, IfStatement},
      literal_expression::LiteralExpression,
      loop_statement::LoopStatement,
      rebind_statement::RebindStatement,
      ret_statement::RetStatement,
      statement::Statement,
      unary_experssion::UnaryExpression,
      var::{
        var_decl::{GlobalDecl, LocalDecl},
        var_ref::VarRef,
      },
    },
    token::{ident::Ident, literal::Literal},
  },
};

struct JitInstructionBlockBuilder<'a> {
  id: BlockId,
  instructions: Vec<JitInstruction<'a>>,
}

impl<'a> JitInstructionBlockBuilder<'a> {
  fn new(id: BlockId) -> Self {
    Self {
      id,
      instructions: Vec::new(),
    }
  }

  fn emit_instr(mut self, instr: JitInstruction<'a>) -> Self {
    self.instructions.push(instr);
    self
  }

  fn terminate_with_instr(self, terminal: JitTerminalInstruction) -> TerminatedBlock<'a> {
    TerminatedBlock {
      id: self.id,
      block: JitInstructionBlock::new(self.instructions, terminal),
    }
  }
}

struct TerminatedBlock<'a> {
  id: BlockId,
  block: JitInstructionBlock<'a>,
}

struct JitFunctionBuilder<'a> {
  entrypoint: BlockId,
  blocks: BlockListBuilder<JitInstructionBlock<'a>>,
  fn_name: &'a Ident,
}

impl<'a> JitFunctionBuilder<'a> {
  fn new(fn_name: &'a Ident) -> Self {
    let mut blocks = BlockListBuilder::new();
    let entrypoint = blocks.allocate_uninitialized();
    Self {
      entrypoint,
      blocks,
      fn_name,
    }
  }

  fn allocate_block(&mut self) -> BlockId {
    self.blocks.allocate_uninitialized()
  }

  fn finish_block(mut self, block: TerminatedBlock<'a>) -> InterpreterResult<Self> {
    self.blocks.set(block.id, block.block)?;
    Ok(self)
  }

  fn build(self) -> InterpreterResult<BlockList<JitInstructionBlock<'a>>> {
    self
      .blocks
      .build()
      .map_err(|err| InterpreterError::jit_err(format!("block was never terminated: {err}",)))
  }
}

// Function compilation state when there is an unterminated instruction block.
struct OpenCursor<'a> {
  fn_builder: JitFunctionBuilder<'a>,
  lexical_scope: JitCompilerLexicalScope<'a>,
  loop_context: LoopContext,
  block: JitInstructionBlockBuilder<'a>,
}

// Function compilation state when all blocks have been terminated.
struct ClosedCursor<'a> {
  fn_builder: JitFunctionBuilder<'a>,
  lexical_scope: JitCompilerLexicalScope<'a>,
  loop_context: LoopContext,
}

#[derive(FromVariants)]
enum Cursor<'a> {
  Open(OpenCursor<'a>),
  Closed(ClosedCursor<'a>),
}

impl<'a> OpenCursor<'a> {
  fn new(fn_name: &'a GlobalDecl) -> Self {
    let fn_builder = JitFunctionBuilder::new(fn_name.name());
    let entrypoint = fn_builder.entrypoint;
    Self {
      fn_builder,
      lexical_scope: JitCompilerLexicalScope::default(),
      block: JitInstructionBlockBuilder::new(entrypoint),
      loop_context: LoopContext::default(),
    }
  }

  fn enter_lexical_scope(self) -> Self {
    Self {
      lexical_scope: self.lexical_scope.enter_block(),
      ..self
    }
  }

  fn exit_lexical_scope(self) -> InterpreterResult<Self> {
    Ok(Self {
      lexical_scope: self.lexical_scope.exit_block()?,
      ..self
    })
  }

  fn allocate_block(&mut self) -> BlockId {
    self.fn_builder.allocate_block()
  }

  fn emit_instr(self, instr: JitInstruction<'a>) -> Self {
    Self {
      block: self.block.emit_instr(instr),
      ..self
    }
  }

  fn return_unit(self) -> InterpreterResult<ClosedCursor<'a>> {
    self
      .emit_instr(JitInstruction::LoadUnit)
      .terminate(JitTerminalInstruction::Return)
  }

  fn terminate(self, terminal: JitTerminalInstruction) -> InterpreterResult<ClosedCursor<'a>> {
    Ok(ClosedCursor {
      fn_builder: self
        .fn_builder
        .finish_block(self.block.terminate_with_instr(terminal))?,
      lexical_scope: self.lexical_scope,
      loop_context: self.loop_context,
    })
  }

  fn emit_local_store(mut self, var: &'a LocalDecl) -> Self {
    let local_id = self.lexical_scope.bind(var.name(), Mutability::Immutable);
    self.emit_instr(JitInstruction::StoreLocal(local_id))
  }

  fn emit_local_mutable_store(mut self, var: &'a LocalDecl) -> Self {
    let local_id = self.lexical_scope.bind(var.name(), Mutability::Mutable);
    self.emit_instr(JitInstruction::StoreLocal(local_id))
  }

  fn emit_local_rebind(self, ref_expr: &'a VarRef) -> InterpreterResult<Self> {
    let info = self
      .lexical_scope
      .get_binding(ref_expr.name())
      .ok_or_else(|| InterpreterError::jit_err(format!("Unknown variable {ref_expr}")))?;
    if !matches!(info.mutability(), Mutability::Mutable) {
      return Err(InterpreterError::jit_err(format!(
        "Cannot rebind immutable variable {ref_expr}"
      )));
    }
    let local_id = info.local_id();
    Ok(self.emit_instr(JitInstruction::StoreLocal(local_id)))
  }

  fn emit_local_load(self, name: &'a Ident) -> Self {
    match self.lexical_scope.get_binding(name) {
      Some(info) => {
        let local_id = info.local_id();
        self.emit_instr(JitInstruction::LoadLocal(local_id))
      }
      None => self.emit_instr(JitInstruction::LoadGlobal(name)),
    }
  }

  fn emit_literal_load(self, literal: &'a Literal) -> Self {
    self.emit_instr(JitInstruction::LoadLiteral(literal))
  }

  fn compile_statement(self, statement: &'a Statement) -> InterpreterResult<Cursor<'a>> {
    match statement {
      Statement::Bind(bind_statement) => Ok(self.compile_bind_statement(bind_statement)?.into()),
      Statement::Rebind(rebind_statement) => {
        Ok(self.compile_rebind_statement(rebind_statement)?.into())
      }
      Statement::Ret(ret_statement) => Ok(self.compile_ret_statement(ret_statement)?.into()),
      Statement::CallStatement(call_expression) => {
        Ok(self.compile_call_expression(call_expression)?.into())
      }
      Statement::IfStatement(if_statement) => Ok(self.compile_if_statement(if_statement)?.into()),
      Statement::Block(block) => self.compile_lexical_block(block),
      Statement::LoopStatement(loop_statement) => {
        Ok(self.compile_loop_statement(loop_statement)?.into())
      }
      Statement::Break => Ok(self.compile_break_statement()?.into()),
    }
  }

  fn compile_bind_statement(self, statement: &'a BindStatement) -> InterpreterResult<Self> {
    let compiled_expr = self.compile_expr(statement.expr())?;
    match statement.mutability() {
      Mutability::Immutable => Ok(compiled_expr.emit_local_store(statement.var())),
      Mutability::Mutable => Ok(compiled_expr.emit_local_mutable_store(statement.var())),
    }
  }

  fn compile_rebind_statement(self, statement: &'a RebindStatement) -> InterpreterResult<Self> {
    self
      .compile_expr(statement.expr())?
      .emit_local_rebind(statement.var())
  }

  fn compile_lexical_block(self, block: &'a Block) -> InterpreterResult<Cursor<'a>> {
    Cursor::from(self.enter_lexical_scope())
      .compile_statements(block.statements())?
      .exit_lexical_scope()
  }

  fn compile_if_statement(
    mut self,
    if_statement: &'a IfStatement,
  ) -> InterpreterResult<OpenCursor<'a>> {
    let if_block_id = self.allocate_block();
    let else_block_id = self.allocate_block();
    let join_block_id = self.allocate_block();

    Ok(
      self
        .compile_expr(if_statement.condition())?
        .terminate(JitTerminalInstruction::ConditionalJump(
          ConditionalJumpTargets::new(if_block_id, else_block_id),
        ))?
        .start_block(if_block_id)
        .compile_lexical_block(if_statement.body())?
        .finish_with_fallthrough_to(join_block_id)?
        .start_block(else_block_id)
        .compile_else_block(if_statement.else_clause())?
        .finish_with_fallthrough_to(join_block_id)?
        .start_block(join_block_id),
    )
  }

  fn compile_loop_statement(
    mut self,
    loop_statement: &'a LoopStatement,
  ) -> InterpreterResult<OpenCursor<'a>> {
    let loop_block_id = self.allocate_block();
    let join_block_id = self.allocate_block();
    Ok(
      self
        .terminate(JitTerminalInstruction::Jump(loop_block_id))?
        .enter_loop_context(join_block_id, |cursor| {
          cursor
            .start_block(loop_block_id)
            .compile_lexical_block(loop_statement.body())?
            .finish_with_fallthrough_to(loop_block_id)
        })?
        .start_block(join_block_id),
    )
  }

  fn compile_break_statement(self) -> InterpreterResult<ClosedCursor<'a>> {
    let break_target = self
      .loop_context
      .break_target()
      .ok_or_else(|| InterpreterError::jit_err("Cannot break outside of a loop"))?;
    self.terminate(JitTerminalInstruction::Jump(break_target))
  }

  fn compile_expr(self, expr: &'a Expression) -> InterpreterResult<Self> {
    match expr.variant() {
      ExpressionVariant::Literal(expr) => self.compile_literal_expression(expr),
      ExpressionVariant::VarRef(expr) => self.compile_var_ref_expression(expr),
      ExpressionVariant::BinaryExpression(expr) => self.compile_binary_expression(expr),
      ExpressionVariant::UnaryExpression(expr) => self.compile_unary_expression(expr),
      ExpressionVariant::CallExpression(expr) => self.compile_call_expression(expr),
      expr => Err(InterpreterError::unimplemented(format!(
        "evaluation of expression not yet implemented: {expr}"
      ))),
    }
  }

  fn compile_var_ref_expression(self, expr: &'a VarRef) -> InterpreterResult<Self> {
    Ok(self.emit_local_load(expr.name()))
  }

  fn compile_literal_expression(self, expr: &'a LiteralExpression) -> InterpreterResult<Self> {
    Ok(self.emit_literal_load(expr.literal()))
  }

  fn compile_else_block(self, else_clause: &'a ElseClause) -> InterpreterResult<Cursor<'a>> {
    match else_clause {
      ElseClause::None => Ok(self.into()),
      ElseClause::Else(block) => self.compile_lexical_block(block),
      ElseClause::ElseIf(if_statement) => Ok(self.compile_if_statement(if_statement)?.into()),
    }
  }

  fn compile_binary_expression(self, expr: &'a BinaryExpression) -> InterpreterResult<Self> {
    Ok(
      self
        .compile_expr(expr.lhs())?
        .compile_expr(expr.rhs())?
        .emit_instr(JitInstruction::BinaryOp(expr.op())),
    )
  }

  fn compile_unary_expression(self, expr: &'a UnaryExpression) -> InterpreterResult<Self> {
    Ok(
      self
        .compile_expr(expr.expr())?
        .emit_instr(JitInstruction::UnaryOp(expr.op())),
    )
  }

  fn compile_ret_statement(
    self,
    ret_statement: &'a RetStatement,
  ) -> InterpreterResult<ClosedCursor<'a>> {
    // Tail calls for direct recursion.
    if let ExpressionVariant::CallExpression(call) = ret_statement.expr().variant()
      && let ExpressionVariant::VarRef(var_ref) = call.target().variant()
      && var_ref.name() == self.fn_builder.fn_name
    {
      let entrypoint = self.fn_builder.entrypoint;
      return self
        .compile_call_arguments(call)?
        .terminate(JitTerminalInstruction::Jump(entrypoint));
    }

    self
      .compile_expr(ret_statement.expr())?
      .terminate(JitTerminalInstruction::Return)
  }

  fn compile_call_expression(
    self,
    call_expression: &'a CallExpression,
  ) -> InterpreterResult<OpenCursor<'a>> {
    Ok(
      self
        .compile_call_arguments(call_expression)?
        .compile_expr(call_expression.target())?
        .emit_instr(JitInstruction::Call(JitCallInstruction::with_arity(
          call_expression.argument_list().len() as u32,
        ))),
    )
  }

  fn compile_call_arguments(
    self,
    call_expression: &'a CallExpression,
  ) -> InterpreterResult<OpenCursor<'a>> {
    call_expression
      .argument_list()
      .iter()
      .try_fold(self, |cursor, expr| cursor.compile_expr(expr))
  }
}

impl<'a> ClosedCursor<'a> {
  fn start_block(self, block_id: BlockId) -> OpenCursor<'a> {
    OpenCursor {
      fn_builder: self.fn_builder,
      lexical_scope: self.lexical_scope,
      loop_context: self.loop_context,
      block: JitInstructionBlockBuilder::new(block_id),
    }
  }

  fn exit_lexical_scope(self) -> InterpreterResult<Self> {
    Ok(Self {
      lexical_scope: self.lexical_scope.exit_block()?,
      ..self
    })
  }

  fn enter_loop_context<F>(
    mut self,
    join_block_id: BlockId,
    context_fn: F,
  ) -> InterpreterResult<Self>
  where
    F: FnOnce(Self) -> InterpreterResult<Self>,
  {
    let prev_loop_context = self.loop_context.exchange(join_block_id);
    let mut cursor = context_fn(self)?;
    cursor.loop_context = prev_loop_context;
    Ok(cursor)
  }
}

impl<'a> Cursor<'a> {
  fn finish_with_fallthrough_to(self, block_id: BlockId) -> InterpreterResult<ClosedCursor<'a>> {
    match self {
      Cursor::Open(cursor) => cursor.terminate(JitTerminalInstruction::Jump(block_id)),
      Cursor::Closed(cursor) => Ok(cursor),
    }
  }

  fn compile_statement(self, statement: &'a Statement) -> InterpreterResult<Self> {
    match self {
      Cursor::Open(cur) => cur.compile_statement(statement),
      Cursor::Closed(_) => Err(InterpreterError::jit_err(format!(
        "jit compilation failed: unreachable statement: {:?}",
        statement
      ))),
    }
  }

  fn compile_statements(
    self,
    statements: impl IntoIterator<Item = &'a Statement>,
  ) -> InterpreterResult<Self> {
    statements
      .into_iter()
      .try_fold(self, |cur, stmt| cur.compile_statement(stmt))
  }

  fn exit_lexical_scope(self) -> InterpreterResult<Self> {
    match self {
      Cursor::Open(cur) => Ok(cur.exit_lexical_scope()?.into()),
      Cursor::Closed(cur) => Ok(cur.exit_lexical_scope()?.into()),
    }
  }

  fn compile_fn_decl(fn_decl: &'a FunctionDecl) -> InterpreterResult<JitCompiledFunction<'a>> {
    let cur = fn_decl
      .parameters()
      .iter()
      .rev()
      .fold(OpenCursor::new(fn_decl.name_decl()), |cursor, param| {
        cursor.emit_local_store(param.var())
      })
      .compile_lexical_block(fn_decl.body())?;

    // Terminate by returning unit value if not already closed.
    let cur = match cur {
      Cursor::Open(cur) => cur.return_unit()?,
      Cursor::Closed(cur) => cur,
    };

    Ok(JitCompiledFunction::new(
      cur.fn_builder.entrypoint,
      cur.fn_builder.build()?,
    ))
  }
}

pub fn compile_to_bytecode<'a>(
  fn_decl: &'a FunctionDecl,
) -> InterpreterResult<JitCompiledFunction<'a>> {
  Cursor::compile_fn_decl(fn_decl)
}

#[cfg(test)]
mod tests {
  use crate::{
    error::JangResult,
    interpreter::{
      bytecode::{
        compiler::{
          compiler::compile_to_bytecode, instruction::matchers::*,
          instruction_block_list::testing::block_id,
        },
        runtime::local_table::testing::local_id,
      },
      error::InterpreterError,
    },
    parser::{
      ast::{binary_expression::BinaryOp, function_decl::FunctionDecl, unary_experssion::UnaryOp},
      grammar::testing::lex_and_parse_jang_file,
      token::{ident::matchers::ident, literal::matchers::integral},
    },
  };
  use googletest::prelude::*;

  fn parse_fn_decl(text: impl IntoIterator<Item = char>) -> JangResult<FunctionDecl> {
    lex_and_parse_jang_file(text)?
      .function_decls()
      .first()
      .cloned()
      .ok_or_else(|| InterpreterError::generic_err("no function decls in AST").into())
  }

  #[gtest]
  fn empty_function() {
    let decl = parse_fn_decl(
      r#"
      fn f() { }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![load_unit_instruction()],
        ret_terminator(),
      )))
    )
  }

  #[gtest]
  fn binary_operators() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        ret 2 * (3 + 4)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("2")),
          load_literal_instruction(integral("3")),
          load_literal_instruction(integral("4")),
          binary_op_instruction(pat!(BinaryOp::Add)),
          binary_op_instruction(pat!(BinaryOp::Mul)),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn unary_operator() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> bool {
        ret !(2 == 3)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("2")),
          load_literal_instruction(integral("3")),
          binary_op_instruction(pat!(BinaryOp::Equal)),
          unary_op_instruction(pat!(UnaryOp::LogicalNot)),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn if_statement() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        if 0 {
          ret 1
        } else {
        }
        ret 2
      }
      "#
      .chars(),
    )
    .unwrap();

    let entry_block_id = block_id(0);
    let if_block_id = block_id(1);
    let else_block_id = block_id(2);
    let join_block_id = block_id(3);

    expect_that!(
      compile_to_bytecode(&decl),
      ok(all![
        has_instruction_block(
          entry_block_id,
          instruction_block(
            elements_are![load_literal_instruction(integral("0"))],
            conditional_jump_terminator(all![
              if_branch_target(eq(&if_block_id)),
              else_branch_target(eq(&else_block_id))
            ])
          ),
        ),
        has_instruction_block(
          if_block_id,
          instruction_block(
            elements_are![load_literal_instruction(integral("1"))],
            ret_terminator()
          )
        ),
        has_instruction_block(
          else_block_id,
          instruction_block(is_empty(), jump_terminator(eq(&join_block_id)))
        ),
        has_instruction_block(
          join_block_id,
          instruction_block(anything(), ret_terminator())
        )
      ])
    )
  }

  #[gtest]
  fn loop_statement() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        loop {
          if x {
            break
          }
        }
      }
      "#
      .chars(),
    )
    .unwrap();

    let entry_block_id = block_id(0);
    let loop_block_id = block_id(1);
    let join_block_id = block_id(2);
    let if_block_id = block_id(3);
    let else_block_id = block_id(4);
    let loop_end_block_id = block_id(5);

    expect_that!(
      compile_to_bytecode(&decl),
      ok(all![
        has_instruction_block(
          entry_block_id,
          instruction_block(is_empty(), jump_terminator(eq(&loop_block_id))),
        ),
        has_instruction_block(
          loop_block_id,
          instruction_block(
            elements_are![load_global_instruction(ident("x"))],
            conditional_jump_terminator(all![
              if_branch_target(eq(&if_block_id)),
              else_branch_target(eq(&else_block_id))
            ])
          )
        ),
        has_instruction_block(
          if_block_id,
          instruction_block(is_empty(), jump_terminator(eq(&join_block_id)))
        ),
        has_instruction_block(
          else_block_id,
          instruction_block(is_empty(), jump_terminator(eq(&loop_end_block_id)))
        ),
        has_instruction_block(
          loop_end_block_id,
          instruction_block(is_empty(), jump_terminator(eq(&loop_block_id)))
        ),
        has_instruction_block(
          join_block_id,
          instruction_block(anything(), ret_terminator())
        )
      ])
    )
  }

  #[gtest]
  fn lexical_scoping() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        let x = 1
        {
          let x = 2
          {
            let x = 3
          }
          ret x
        }
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          store_local_instruction(eq(&local_id(0))),
          load_literal_instruction(integral("2")),
          store_local_instruction(eq(&local_id(1))),
          load_literal_instruction(integral("3")),
          store_local_instruction(eq(&local_id(2))),
          load_local_instruction(eq(&local_id(1))),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn function_call_no_args() {
    let decl = parse_fn_decl(
      r#"
      fn f() {
        func()
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_global_instruction(ident("func")),
          call_instruction(call_with_arity(eq(&0))),
          load_unit_instruction()
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn store_and_load_local() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        let x = 1 + 2
        ret x
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          load_literal_instruction(integral("2")),
          binary_op_instruction(pat!(BinaryOp::Add)),
          store_local_instruction(eq(&local_id(0))),
          load_local_instruction(eq(&local_id(0))),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn store_and_load_mutable_local() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        mut x = 1 + 2
        ret x
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          load_literal_instruction(integral("2")),
          binary_op_instruction(pat!(BinaryOp::Add)),
          store_local_instruction(eq(&local_id(0))),
          load_local_instruction(eq(&local_id(0))),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn rebind_mutable_local() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        mut x = 1
        x = 70
        ret x
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          store_local_instruction(eq(&local_id(0))),
          load_literal_instruction(integral("70")),
          store_local_instruction(eq(&local_id(0))),
          load_local_instruction(eq(&local_id(0))),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn cannot_rebind_immutable_local() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        let x = 1
        x = 70
        ret x
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(compile_to_bytecode(&decl), err(anything()))
  }

  #[gtest]
  fn cannot_rebind_unknown_local() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        mut x = 1
        y = 70
        ret x
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(compile_to_bytecode(&decl), err(anything()))
  }

  #[gtest]
  fn call_with_multiple_args() {
    let decl = parse_fn_decl(
      r#"
      fn f() {
        func(1, 2, 3)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          load_literal_instruction(integral("2")),
          load_literal_instruction(integral("3")),
          load_global_instruction(ident("func")),
          call_instruction(call_with_arity(eq(&3))),
          load_unit_instruction(),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn call_argument_eval_order() {
    let decl = parse_fn_decl(
      r#"
      fn f() {
        func(1 + 2, 3 + 4)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_literal_instruction(integral("1")),
          load_literal_instruction(integral("2")),
          binary_op_instruction(pat!(BinaryOp::Add)),
          load_literal_instruction(integral("3")),
          load_literal_instruction(integral("4")),
          binary_op_instruction(pat!(BinaryOp::Add)),
          load_global_instruction(ident("func")),
          call_instruction(call_with_arity(eq(&2))),
          load_unit_instruction()
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn fn_decl_with_parameters() {
    let decl = parse_fn_decl(
      r#"
      fn f(a: i64, b: i64, c: i64) {
        ret a + (b + c)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          // Load arguments.
          store_local_instruction(eq(&local_id(0))),
          store_local_instruction(eq(&local_id(1))),
          store_local_instruction(eq(&local_id(2))),
          // load a
          load_local_instruction(eq(&local_id(2))),
          // b + c
          load_local_instruction(eq(&local_id(1))),
          load_local_instruction(eq(&local_id(0))),
          binary_op_instruction(pat!(BinaryOp::Add)),
          binary_op_instruction(pat!(BinaryOp::Add)),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn values_as_functions() {
    let decl = parse_fn_decl(
      r#"
      fn f() {
        let x = global_fn
        ret x(1)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          load_global_instruction(ident("global_fn")),
          store_local_instruction(eq(&local_id(0))),
          load_literal_instruction(integral("1")),
          load_local_instruction(eq(&local_id(0))),
          call_instruction(call_with_arity(eq(&1))),
        ],
        ret_terminator()
      )))
    )
  }

  #[gtest]
  fn simple_tail_recursion_no_args() {
    let decl = parse_fn_decl(
      r#"
      fn f() -> i64 {
        ret f()
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![],
        jump_terminator(eq(&block_id(0))),
      )))
    )
  }

  #[gtest]
  fn simple_tail_recursion_with_args() {
    let decl = parse_fn_decl(
      r#"
      fn f(a: i64, b: i64) -> i64 {
        ret f(a, b)
      }
      "#
      .chars(),
    )
    .unwrap();

    expect_that!(
      compile_to_bytecode(&decl),
      ok(entry_block(instruction_block(
        elements_are![
          store_local_instruction(eq(&local_id(0))),
          store_local_instruction(eq(&local_id(1))),
          load_local_instruction(eq(&local_id(1))),
          load_local_instruction(eq(&local_id(0))),
        ],
        jump_terminator(eq(&block_id(0))),
      )))
    )
  }
}
