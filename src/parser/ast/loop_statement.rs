use std::fmt::Display;

use crate::parser::ast::block::Block;

#[derive(Clone, Debug)]
pub struct LoopStatement {
  body: Block,
}

impl LoopStatement {
  pub fn new(body: Block) -> Self {
    Self { body }
  }
}

impl Display for LoopStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "loop {}", self.body)
  }
}
