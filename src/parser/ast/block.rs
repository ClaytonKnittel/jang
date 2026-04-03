use std::fmt::Display;

use crate::parser::ast::statement::Statement;

#[derive(Default)]
pub struct BlockBuilder {
  statements: Vec<Statement>,
}

impl BlockBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn push_statement(mut self, statement: impl Into<Statement>) -> Self {
    self.statements.push(statement.into());
    self
  }

  pub fn build(self) -> Block {
    Block::new(self.statements)
  }
}

#[derive(Clone, Debug)]
pub struct Block {
  statements: Vec<Statement>,
}

impl Block {
  fn new(statements: Vec<Statement>) -> Self {
    Self { statements }
  }

  pub fn statements(&self) -> &[Statement] {
    &self.statements
  }
}

impl Display for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{{")?;
    for statement in &self.statements {
      writeln!(f, "{statement}")?;
    }
    write!(f, "}}")
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{block::Block, statement::Statement};
  use googletest::prelude::*;

  pub fn block<'a>(statements_matcher: impl Matcher<&'a [Statement]>) -> impl Matcher<&'a Block> {
    property!(&Block.statements(), statements_matcher)
  }

  pub fn block_statement<'a>(
    statements_matcher: impl Matcher<&'a [Statement]>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Block(block(statements_matcher)))
  }
}
