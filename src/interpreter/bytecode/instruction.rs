use crate::parser::{
  ast::binary_expression::BinaryOp,
  token::{ident::Ident, literal::Literal},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct LocalId(u32);
impl LocalId {
  pub fn next(self) -> Self {
    Self(self.0 + 1)
  }

  pub fn zero() -> Self {
    LocalId(0)
  }
}

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
  // sets the stack base pointer,
  // and jumps to instructions for the desired function.
  Call(CallInstr),

  // Jumps to a block of instructions within the current function.
  Jump(BlockId),

  // Pops a value of the stack, jumps based on its true/false value.
  ConditionalJump(ConditionalJumpTargets),

  // Pops a value off the stack, removes the entire function's stack frame,
  // and pushes the value onto the stack.
  Ret,
}

pub struct ConditionalJumpTargets {
  pub true_target: BlockId,
  pub false_target: BlockId,
}

pub struct CallInstr {
  pub arity: u32,
}

// A block is a sequence of instructions that are always executed
// in order and without interruption.
#[derive(Default)]
pub struct JitInstructionBlock<'a> {
  pub instructions: Vec<JitInstruction<'a>>,
}

pub struct JitCompiledFunction<'a> {
  pub blocks: Vec<JitInstructionBlock<'a>>,
}
