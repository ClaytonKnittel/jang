use std::collections::HashMap;

use crate::{
  error::{JangError, JangResult},
  interpreter::bytecode::instruction::{
    BlockId, CallInstr, ConditionalJumpTargets, JitCompiledFunction, JitInstruction,
    JitInstructionBlock, LocalId,
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

struct JitCompilerLexicalBlock<'a> {
  next_local_id: LocalId,
  locals: HashMap<&'a Ident, LocalId>,
}

impl<'a> JitCompilerLexicalBlock<'a> {
  fn get_binding(&self, name: &Ident) -> Option<LocalId> {
    self.locals.get(name).cloned()
  }

  fn create_binding(&mut self, name: &'a Ident) -> LocalId {
    let id = self.next_local_id;
    self.next_local_id = self.next_local_id.next();
    self.locals.insert(name, id);
    id
  }
}

struct JitCompilerLexicalScope<'a> {
  parent_blocks: Vec<JitCompilerLexicalBlock<'a>>,
  current_block: JitCompilerLexicalBlock<'a>,

  max_size: usize,
}

impl<'a> JitCompilerLexicalScope<'a> {
  fn new() -> Self {
    Self {
      parent_blocks: Vec::new(),
      current_block: JitCompilerLexicalBlock {
        next_local_id: LocalId::zero(),
        locals: HashMap::new(),
      },
      max_size: 0,
    }
  }

  fn get_binding(&self, name: &Ident) -> Option<LocalId> {
    std::iter::once(&self.current_block)
      .chain(self.parent_blocks.iter().rev())
      .find_map(|lexical_block| lexical_block.get_binding(name))
  }

  fn create_binding(&mut self, name: &'a Ident) -> LocalId {
    let id = self.current_block.create_binding(name);
    self.max_size = self.max_size.max(id.as_index() + 1);
    id
  }

  fn push_block(mut self) -> Self {
    let next_local_id = self.current_block.next_local_id;
    self.parent_blocks.push(self.current_block);
    self.current_block = JitCompilerLexicalBlock {
      next_local_id,
      locals: Default::default(),
    };
    self
  }

  fn pop_block(&mut self) -> JangResult {
    self.current_block = self
      .parent_blocks
      .pop()
      .ok_or_else(|| JangError::interpret_error("unexpected pop of lexical scope"))?;
    Ok(())
  }
}

struct JitCompilationState<'a> {
  next_block_id: BlockId,

  blocks: Vec<JitInstructionBlock<'a>>,
  current_block_id: BlockId,

  lexical_scope: JitCompilerLexicalScope<'a>,
}

impl<'a> JitCompilationState<'a> {
  fn new() -> Self {
    let current_block_id = BlockId::zero();
    Self {
      next_block_id: current_block_id.next(),
      current_block_id,
      blocks: vec![JitInstructionBlock::default()],
      lexical_scope: JitCompilerLexicalScope::new(),
    }
  }

  fn emit_instr(mut self, instr: JitInstruction<'a>) -> Self {
    self
      .blocks
      .get_mut(self.current_block_id.as_index())
      .expect("internal jit failure: could not find block")
      .instructions
      .push(instr);
    self
  }

  fn new_block(&mut self) -> BlockId {
    let id = self.next_block_id;
    self.next_block_id = id.next();
    self.blocks.push(JitInstructionBlock::default());
    id
  }

  fn switch_to_block(mut self, block_id: BlockId) -> Self {
    self.current_block_id = block_id;
    self
  }

  fn bind_local(mut self, name: &'a Ident) -> Self {
    let local_id = self.lexical_scope.create_binding(name);
    self.emit_instr(JitInstruction::StoreLocal(local_id))
  }

  fn emit_local_load(self, name: &'a Ident) -> Self {
    if let Some(local_id) = self.lexical_scope.get_binding(name) {
      self.emit_instr(JitInstruction::LoadLocal(local_id))
    } else {
      self.emit_instr(JitInstruction::LoadGlobal(name))
    }
  }

  fn emit_literal_load(self, literal: &'a Literal) -> Self {
    self.emit_instr(JitInstruction::LoadLiteral(literal))
  }

  fn enter_lexical_scope(mut self) -> Self {
    self.lexical_scope = self.lexical_scope.push_block();
    self
  }

  fn exit_lexical_scope(mut self) -> JangResult<Self> {
    self.lexical_scope.pop_block()?;
    Ok(self)
  }

  fn compile_fn_decl(mut self, fn_decl: &'a FunctionDecl) -> JangResult<JitCompiledFunction<'a>> {
    self = fn_decl
      .parameters()
      .iter()
      .fold(self, |s, param| s.bind_local(param.name()))
      .compile_lexical_block(fn_decl.body())?
      .emit_instr(JitInstruction::Ret);

    Ok(JitCompiledFunction {
      entrypoint: BlockId::zero(),
      locals_size: self.lexical_scope.max_size,
      blocks: self.blocks,
      fn_decl,
    })
  }

  fn compile_statement(self, statement: &'a Statement) -> JangResult<Self> {
    match statement {
      Statement::Let(let_statement) => Ok(
        self
          .compile_expr(let_statement.expr())?
          .bind_local(let_statement.var()),
      ),
      Statement::Ret(ret_statement) => Ok(
        self
          .compile_expr(ret_statement.expr())?
          .emit_instr(JitInstruction::RetWithValue),
      ),
      Statement::CallStatement(call_expression) => self.compile_call_expression(call_expression),
      Statement::IfStatement(if_statement) => self.compile_if_statement(if_statement),
      Statement::Block(block) => self.compile_lexical_block(block),
      Statement::LoopStatement(_) => Err(JangError::interpret_error("loops are not yet supported")),
      Statement::Break => Err(JangError::interpret_error("break is not yet supported")),
    }
  }

  fn compile_lexical_block(self, block: &'a Block) -> JangResult<Self> {
    block
      .statements()
      .iter()
      .try_fold(self.enter_lexical_scope(), |s, statement| {
        s.compile_statement(statement)
      })?
      .exit_lexical_scope()
  }

  fn compile_if_statement(mut self, if_statement: &'a IfStatement) -> JangResult<Self> {
    let (if_block, else_block, join_block) = (self.new_block(), self.new_block(), self.new_block());
    Ok(
      self
        // Compile the condition and jump to the appropriate block.
        .compile_expr(if_statement.condition())?
        .emit_instr(JitInstruction::ConditionalJump(ConditionalJumpTargets {
          true_target: if_block,
          false_target: else_block,
        }))
        // Compile the if body.
        .switch_to_block(if_block)
        .compile_lexical_block(if_statement.body())?
        .emit_instr(JitInstruction::Jump(join_block))
        // Compile the else body.
        .switch_to_block(else_block)
        .compile_else_block(if_statement.else_clause())?
        .emit_instr(JitInstruction::Jump(join_block))
        // Return to the main instruction path for the caller.
        .switch_to_block(join_block),
    )
  }

  fn compile_else_block(self, else_clause: &'a ElseClause) -> JangResult<Self> {
    Ok(match else_clause {
      ElseClause::None => self,
      ElseClause::Else(block) => self.compile_lexical_block(block)?,
      ElseClause::ElseIf(if_statement) => self.compile_if_statement(if_statement)?,
    })
  }

  fn compile_expr(self, expr: &'a Expression) -> JangResult<Self> {
    match expr {
      Expression::Literal(literal) => Ok(self.emit_literal_load(literal)),
      Expression::Ident(ident) => Ok(self.emit_local_load(ident)),
      Expression::BinaryExpression(expr) => self.compile_binary_expression(expr),
      Expression::CallExpression(expr) => self.compile_call_expression(expr),
      Expression::DotExpression(_) => Err(JangError::interpret_error(
        "dot expression not yet supported",
      )),
    }
  }

  fn compile_binary_expression(self, expr: &'a BinaryExpression) -> JangResult<Self> {
    Ok(
      self
        .compile_expr(expr.rhs())?
        .compile_expr(expr.lhs())?
        .emit_instr(JitInstruction::BinaryOp(expr.op())),
    )
  }

  fn compile_call_expression(self, call_expression: &'a CallExpression) -> JangResult<Self> {
    Ok(
      call_expression
        .argument_list()
        .iter()
        .rev()
        .try_fold(self, |s, expr| s.compile_expr(expr))?
        .compile_expr(call_expression.target())?
        .emit_instr(JitInstruction::Call(CallInstr {
          arity: call_expression.argument_list().len() as u32,
        })),
    )
  }
}

pub fn compile_to_bytecode<'a>(fn_decl: &'a FunctionDecl) -> JangResult<JitCompiledFunction<'a>> {
  JitCompilationState::new().compile_fn_decl(fn_decl)
}
