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

  pub fn body(&self) -> &Block {
    &self.body
  }
}

impl Display for LoopStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "loop {}", self.body)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    block::matchers::block, loop_statement::LoopStatement, statement::Statement,
  };
  use googletest::prelude::*;

  pub fn loop_statement<'a>(
    body_matcher: impl Matcher<&'a [Statement]>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::LoopStatement(pat!(LoopStatement {
      body: block(body_matcher)
    })))
  }
}
