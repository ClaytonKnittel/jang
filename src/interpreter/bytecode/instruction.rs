use crate::{
  interpreter::bytecode::local_table::LocalId,
  parser::{
    ast::{binary_expression::BinaryOp, function_decl::FunctionDecl},
    token::{ident::Ident, literal::Literal},
  },
};

// The ID of a block within a function.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct BlockId(u32);
impl BlockId {
  pub fn next(self) -> Self {
    Self(self.0 + 1)
  }

  pub fn zero() -> Self {
    BlockId(0)
  }

  pub fn as_index(&self) -> usize {
    self.0 as usize
  }
}

#[derive(Debug)]
pub enum JitInstruction<'a> {
  // Binary operator. Pops lhs off the stack, pops rhs off the stack,
  // combines the two, and pushes the result on the stack.
  BinaryOp(BinaryOp),

  // Push a literal value onto the stack.
  LoadLiteral(&'a Literal),

  // Pushes a global value onto the stack.
  LoadGlobal(&'a Ident),

  // Pushes a local onto the stack.
  LoadLocal(LocalId),

  // Pops a value off the stack and stores as a local.
  StoreLocal(LocalId),

  // Pops a function value off the stack,
  // and jumps to instructions for the desired function.
  Call(CallInstr),
}

#[derive(Debug)]
pub enum JitTerminalInstruction {
  // Jumps to a block of instructions within the current function.
  Jump(BlockId),

  // Pops a value of the stack, jumps based on its true/false value.
  ConditionalJump(ConditionalJumpTargets),

  // Returns without a value.
  Ret,

  // Pops a value off the stack and returns it.
  RetWithValue,
}

#[derive(Debug)]
pub struct ConditionalJumpTargets {
  pub true_target: BlockId,
  pub false_target: BlockId,
}

#[derive(Debug)]
pub struct CallInstr {
  pub arity: u32,
}

// A block is a sequence of instructions that are always executed
// in order and without interruption, followed by a terminal instruction.
#[derive(Debug)]
pub struct JitInstructionBlock<'a> {
  instructions: Vec<JitInstruction<'a>>,
  terminator: JitTerminalInstruction,
}

impl<'a> JitInstructionBlock<'a> {
  pub fn new(instructions: Vec<JitInstruction<'a>>, terminator: JitTerminalInstruction) -> Self {
    Self {
      instructions,
      terminator,
    }
  }

  pub fn instructions(&self) -> &[JitInstruction<'a>] {
    &self.instructions
  }

  pub fn terminator(&self) -> &JitTerminalInstruction {
    &self.terminator
  }
}

#[derive(Debug)]
pub struct JitCompiledFunction<'a> {
  entrypoint: BlockId,
  blocks: Vec<JitInstructionBlock<'a>>,
  fn_decl: &'a FunctionDecl,
}

impl<'a> JitCompiledFunction<'a> {
  pub fn new(
    entrypoint: BlockId,
    blocks: Vec<JitInstructionBlock<'a>>,
    fn_decl: &'a FunctionDecl,
  ) -> Self {
    Self {
      entrypoint,
      blocks,
      fn_decl,
    }
  }

  pub fn entrypoint(&self) -> BlockId {
    self.entrypoint
  }

  pub fn block(&self, block_id: BlockId) -> &JitInstructionBlock<'a> {
    &self.blocks[block_id.0 as usize]
  }

  pub fn decl(&self) -> &'a FunctionDecl {
    self.fn_decl
  }
}
