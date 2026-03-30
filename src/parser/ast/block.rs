use crate::parser::ast::statement::{NonRetStatement, RetStatement};

#[derive(Default)]
pub struct BlockBuilder {
  statements: Vec<NonRetStatement>,
}

impl BlockBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_statement(mut self, statement: impl Into<NonRetStatement>) -> Self {
    self.statements.push(statement.into());
    self
  }

  pub fn build(self) -> Block {
    Block::new(self.statements, None)
  }

  pub fn build_with_ret(self, ret_statement: RetStatement) -> Block {
    Block::new(self.statements, Some(ret_statement))
  }
}

#[derive(Clone, Debug)]
pub struct Block {
  statements: Vec<NonRetStatement>,
  ret_statement: Option<RetStatement>,
}

impl Block {
  fn new(statements: Vec<NonRetStatement>, ret_statement: Option<RetStatement>) -> Self {
    Self {
      statements,
      ret_statement,
    }
  }

  pub fn statements(&self) -> &[NonRetStatement] {
    &self.statements
  }

  pub fn ret_statement(&self) -> Option<&RetStatement> {
    self.ret_statement.as_ref()
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::ast::{
    block::Block,
    statement::{NonRetStatement, RetStatement},
  };
  use googletest::prelude::*;

  pub fn block<'a>(
    statements_matcher: impl Matcher<&'a [NonRetStatement]>,
  ) -> impl Matcher<&'a Block> {
    property!(&Block.statements(), statements_matcher)
  }

  pub fn block_with_ret<'a>(
    statements_matcher: impl Matcher<&'a [NonRetStatement]>,
    ret_matcher: impl Matcher<&'a RetStatement>,
  ) -> impl Matcher<&'a Block> {
    all!(
      property!(&Block.statements(), statements_matcher),
      property!(&Block.ret_statement(), some(ret_matcher))
    )
  }
}
