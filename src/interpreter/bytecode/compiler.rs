use std::{collections::HashMap, rc::Rc};

use crate::{
  interpreter::{
    bytecode::{
      instruction::{
        ConditionalJumpTargets, JitCallInstructionBuilder, JitCompiledFunction, JitInstruction,
        JitInstructionBlock, JitTerminalInstruction,
      },
      instruction_block_list::{BlockId, BlockList, BlockListBuilder},
      local_table::LocalId,
    },
    error::{InterpreterError, InterpreterResult},
  },
  parser::{
    ast::{
      binary_expression::BinaryExpression,
      block::Block,
      call_expression::CallExpression,
      expression::Expression,
      function_decl::FunctionDecl,
      if_statement::{ElseClause, IfStatement},
      statement::Statement,
    },
    token::{ident::Ident, literal::Literal},
  },
};

#[derive(Clone)]
struct JitCompilerLexicalScope<'a>(Rc<JitCompilerLexicalBlock<'a>>);

struct JitCompilerLexicalBlock<'a> {
  parent: Option<JitCompilerLexicalScope<'a>>,
  next_local_id: LocalId,
  locals: HashMap<&'a Ident, LocalId>,
}

impl<'a> JitCompilerLexicalScope<'a> {
  fn new() -> Self {
    Self(Rc::new(JitCompilerLexicalBlock {
      parent: None,
      next_local_id: LocalId::default(),
      locals: HashMap::new(),
    }))
  }

  fn get_binding(&self, name: &Ident) -> Option<LocalId> {
    self
      .0
      .locals
      .get(name)
      .copied()
      .or_else(|| self.0.parent.as_ref()?.get_binding(name))
  }

  fn bind(&self, name: &'a Ident) -> (Self, LocalId) {
    let local_id = self.0.next_local_id;
    let mut locals = self.0.locals.clone();
    locals.insert(name, local_id);

    (
      Self(Rc::new(JitCompilerLexicalBlock {
        parent: self.0.parent.clone(),
        next_local_id: local_id.next(),
        locals,
      })),
      local_id,
    )
  }

  fn push_block(&self) -> Self {
    Self(Rc::new(JitCompilerLexicalBlock {
      parent: Some(self.clone()),
      next_local_id: self.0.next_local_id,
      locals: HashMap::new(),
    }))
  }
}

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
}

impl<'a> JitFunctionBuilder<'a> {
  fn new() -> Self {
    let mut blocks = BlockListBuilder::new();
    let entrypoint = blocks.allocate_uninitialized();
    Self { entrypoint, blocks }
  }

  fn allocate_block(&mut self) -> BlockId {
    self.blocks.allocate_uninitialized()
  }

  fn finish_block(mut self, finished: TerminatedBlock<'a>) -> InterpreterResult<Self> {
    self.blocks.set(finished.id, finished.block)?;
    Ok(self)
  }

  fn into_blocks(self) -> InterpreterResult<BlockList<JitInstructionBlock<'a>>> {
    self
      .blocks
      .build()
      .map_err(|err| InterpreterError::jit_err(format!("block was never terminated: {err}",)))
  }
}

// Function compilation state when there is an open instruction block.
struct OpenCursor<'a> {
  fn_builder: JitFunctionBuilder<'a>,
  lexical_scope: JitCompilerLexicalScope<'a>,
  block: JitInstructionBlockBuilder<'a>,
}

// Function compilation state when all blocks have been terminated.
struct ClosedCursor<'a> {
  fn_builder: JitFunctionBuilder<'a>,
  lexical_scope: JitCompilerLexicalScope<'a>,
}

enum Cursor<'a> {
  Open(OpenCursor<'a>),
  Closed(ClosedCursor<'a>),
}

impl<'a> From<OpenCursor<'a>> for Cursor<'a> {
  fn from(val: OpenCursor<'a>) -> Self {
    Cursor::Open(val)
  }
}

impl<'a> From<ClosedCursor<'a>> for Cursor<'a> {
  fn from(val: ClosedCursor<'a>) -> Self {
    Cursor::Closed(val)
  }
}

impl<'a> OpenCursor<'a> {
  fn new() -> Self {
    let fn_builder = JitFunctionBuilder::new();
    let entrypoint = fn_builder.entrypoint;
    Self {
      fn_builder,
      lexical_scope: JitCompilerLexicalScope::new(),
      block: JitInstructionBlockBuilder::new(entrypoint),
    }
  }

  fn with_lexical_scope(self, lexical_scope: JitCompilerLexicalScope<'a>) -> Self {
    Self {
      lexical_scope,
      ..self
    }
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

  fn terminate(self, terminal: JitTerminalInstruction) -> InterpreterResult<ClosedCursor<'a>> {
    Ok(ClosedCursor {
      fn_builder: self
        .fn_builder
        .finish_block(self.block.terminate_with_instr(terminal))?,
      lexical_scope: self.lexical_scope,
    })
  }

  fn bind_local(self, name: &'a Ident) -> Self {
    let (lexical_scope, local_id) = self.lexical_scope.bind(name);
    self
      .with_lexical_scope(lexical_scope)
      .emit_instr(JitInstruction::StoreLocal(local_id))
  }

  fn emit_local_load(self, name: &'a Ident) -> Self {
    match self.lexical_scope.get_binding(name) {
      Some(local_id) => self.emit_instr(JitInstruction::LoadLocal(local_id)),
      None => self.emit_instr(JitInstruction::LoadGlobal(name)),
    }
  }

  fn emit_literal_load(self, literal: &'a Literal) -> Self {
    self.emit_instr(JitInstruction::LoadLiteral(literal))
  }

  fn compile_statement(self, statement: &'a Statement) -> InterpreterResult<Cursor<'a>> {
    match statement {
      Statement::Let(let_statement) => Ok(
        self
          .compile_expr(let_statement.expr())?
          .bind_local(let_statement.var())
          .into(),
      ),
      Statement::Ret(ret_statement) => Ok(
        self
          .compile_expr(ret_statement.expr())?
          .terminate(JitTerminalInstruction::RetWithValue)?
          .into(),
      ),
      Statement::CallStatement(call_expression) => {
        Ok(self.compile_call_expression(call_expression)?.into())
      }
      Statement::IfStatement(if_statement) => Ok(self.compile_if_statement(if_statement)?.into()),
      Statement::Block(block) => self.compile_lexical_block(block),
      Statement::LoopStatement(_) => {
        Err(InterpreterError::unimplemented("not yet implemented: loop"))
      }
      Statement::Break => Err(InterpreterError::unimplemented(
        "not yet implemented: break",
      )),
    }
  }

  fn compile_lexical_block(self, block: &'a Block) -> InterpreterResult<Cursor<'a>> {
    let outer_scope = self.lexical_scope.clone();
    let inner_scope = self.lexical_scope.push_block();
    Ok(
      block
        .statements()
        .iter()
        .try_fold(
          self.with_lexical_scope(inner_scope).into(),
          |cur: Cursor<'a>, stmt| cur.compile_statement(stmt),
        )?
        .with_lexical_scope(outer_scope),
    )
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
          ConditionalJumpTargets {
            true_target: if_block_id,
            false_target: else_block_id,
          },
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

  fn compile_expr(self, expr: &'a Expression) -> InterpreterResult<Self> {
    match expr {
      Expression::Literal(literal) => Ok(self.emit_literal_load(literal)),
      Expression::Ident(ident) => Ok(self.emit_local_load(ident)),
      Expression::BinaryExpression(expr) => self.compile_binary_expression(expr),
      Expression::CallExpression(expr) => self.compile_call_expression(expr),
      Expression::DotExpression(_) => Err(InterpreterError::unimplemented(
        "dot expression not yet supported",
      )),
    }
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
        .compile_expr(expr.rhs())?
        .compile_expr(expr.lhs())?
        .emit_instr(JitInstruction::BinaryOp(expr.op())),
    )
  }

  fn compile_call_expression(self, call_expression: &'a CallExpression) -> InterpreterResult<Self> {
    Ok(
      call_expression
        .argument_list()
        .iter()
        .rev()
        .try_fold(self, |cursor, expr| cursor.compile_expr(expr))?
        .compile_expr(call_expression.target())?
        .emit_instr(JitInstruction::Call(
          JitCallInstructionBuilder::default()
            .with_arity(call_expression.argument_list().len() as u32)
            .build()
            .expect("internal jit error: incomplete builder"),
        )),
    )
  }
}

impl<'a> ClosedCursor<'a> {
  fn with_lexical_scope(self, lexical_scope: JitCompilerLexicalScope<'a>) -> Self {
    Self {
      lexical_scope,
      ..self
    }
  }

  fn start_block(self, block_id: BlockId) -> OpenCursor<'a> {
    OpenCursor {
      fn_builder: self.fn_builder,
      lexical_scope: self.lexical_scope,
      block: JitInstructionBlockBuilder::new(block_id),
    }
  }
}

impl<'a> Cursor<'a> {
  fn with_lexical_scope(self, lexical_scope: JitCompilerLexicalScope<'a>) -> Self {
    match self {
      Cursor::Open(cursor) => Cursor::Open(cursor.with_lexical_scope(lexical_scope)),
      Cursor::Closed(cursor) => Cursor::Closed(cursor.with_lexical_scope(lexical_scope)),
    }
  }

  fn finish_with_fallthrough_to(self, block_id: BlockId) -> InterpreterResult<ClosedCursor<'a>> {
    match self {
      Cursor::Open(cursor) => cursor.terminate(JitTerminalInstruction::Jump(block_id)),
      Cursor::Closed(cursor) => Ok(cursor),
    }
  }

  fn compile_statement(self, statement: &'a Statement) -> InterpreterResult<Cursor<'a>> {
    match self {
      Cursor::Open(cur) => cur.compile_statement(statement),
      Cursor::Closed(_) => Err(InterpreterError::jit_err(format!(
        "jit compilation failed: unreachable statement: {:?}",
        statement
      ))),
    }
  }

  fn compile_fn_decl(fn_decl: &'a FunctionDecl) -> InterpreterResult<JitCompiledFunction<'a>> {
    let cur = fn_decl
      .parameters()
      .iter()
      .fold(OpenCursor::new(), |cursor, param| {
        cursor.bind_local(param.name())
      })
      .compile_lexical_block(fn_decl.body())?;

    // Terminate with an empty ret if not already closed.
    let cur = match cur {
      Cursor::Open(cur) => cur.terminate(JitTerminalInstruction::Ret)?,
      Cursor::Closed(cur) => cur,
    };

    Ok(JitCompiledFunction::new(
      cur.fn_builder.entrypoint,
      cur.fn_builder.into_blocks()?,
      fn_decl,
    ))
  }
}

pub fn compile_to_bytecode<'a>(
  fn_decl: &'a FunctionDecl,
) -> InterpreterResult<JitCompiledFunction<'a>> {
  Cursor::compile_fn_decl(fn_decl)
}
