use crate::interpreter::bytecode::compiler::instruction_block_list::BlockId;

#[derive(Default)]
pub struct LoopContext {
  break_target: Option<BlockId>,
}

impl LoopContext {
  pub fn new(break_target: BlockId) -> Self {
    Self {
      break_target: Some(break_target),
    }
  }

  pub fn break_target(&self) -> Option<BlockId> {
    self.break_target
  }

  pub fn exchange(&mut self, break_target: BlockId) -> Self {
    Self {
      break_target: self.break_target.replace(break_target),
    }
  }
}
