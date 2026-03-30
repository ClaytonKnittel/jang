use crate::parser::ast::statement::Statement;

#[derive(Clone, Debug)]
pub struct Block {
  statements: Vec<Statement>,
}

impl Block {
  pub fn new(statements: impl Into<Vec<Statement>>) -> Self {
    Self {
      statements: statements.into(),
    }
  }

  pub fn statements(&self) -> &[Statement] {
    &self.statements
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::ast::{block::Block, statement::Statement};
  use googletest::prelude::*;

  pub fn block<'a>(matcher: impl Matcher<&'a [Statement]>) -> impl Matcher<&'a Block> {
    property!(&Block.statements(), matcher)
  }
}
