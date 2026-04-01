use std::fmt::Display;

use crate::parser::ast::statement::{NonRetStatement, RetStatement};

#[derive(Default)]
pub struct BlockBuilder {
  statements: Vec<NonRetStatement>,
}

impl BlockBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn push_statement(mut self, statement: impl Into<NonRetStatement>) -> Self {
    self.statements.push(statement.into());
    self
  }

  pub fn build(self) -> NonRetBlock {
    NonRetBlock::new(self.statements)
  }

  pub fn build_with_ret(self, ret_statement: RetStatement) -> RetBlock {
    RetBlock::new(self.statements, ret_statement)
  }
}

#[derive(Clone, Debug)]
pub struct NonRetBlock {
  statements: Vec<NonRetStatement>,
}

impl NonRetBlock {
  fn new(statements: Vec<NonRetStatement>) -> Self {
    Self { statements }
  }

  pub fn statements(&self) -> &[NonRetStatement] {
    &self.statements
  }
}

impl Display for NonRetBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{{")?;
    for statement in &self.statements {
      writeln!(f, "{statement}")?;
    }
    writeln!(f, "}}")
  }
}

#[derive(Clone, Debug)]
pub struct RetBlock {
  statements: Vec<NonRetStatement>,
  ret_statement: RetStatement,
}

impl RetBlock {
  fn new(statements: Vec<NonRetStatement>, ret_statement: RetStatement) -> Self {
    Self {
      statements,
      ret_statement,
    }
  }

  pub fn statements(&self) -> &[NonRetStatement] {
    &self.statements
  }

  pub fn ret_statement(&self) -> &RetStatement {
    &self.ret_statement
  }
}

impl Display for RetBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{{")?;
    for statement in &self.statements {
      writeln!(f, "{statement}")?;
    }
    writeln!(f, "{}", self.ret_statement)?;
    writeln!(f, "}}")
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

impl From<NonRetBlock> for Block {
  fn from(value: NonRetBlock) -> Self {
    Self::new(value.statements, None)
  }
}

impl From<RetBlock> for Block {
  fn from(value: RetBlock) -> Self {
    Self::new(value.statements, Some(value.ret_statement))
  }
}

impl Display for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{{")?;
    for statement in &self.statements {
      writeln!(f, "{statement}")?;
    }
    if let Some(ret_stmt) = &self.ret_statement {
      writeln!(f, "{ret_stmt}")?;
    }
    writeln!(f, "}}")
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    block::{Block, NonRetBlock, RetBlock},
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

  pub fn non_ret_block<'a>(
    statements_matcher: impl Matcher<&'a [NonRetStatement]>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::Block(property!(
      &NonRetBlock.statements(),
      statements_matcher
    )))
  }

  pub fn ret_block<'a>(
    statements_matcher: impl Matcher<&'a [NonRetStatement]>,
    ret_matcher: impl Matcher<&'a RetStatement>,
  ) -> impl Matcher<&'a RetStatement> {
    pat!(RetStatement::Block(result_of!(
      Box::as_ref,
      all!(
        property!(&RetBlock.statements(), statements_matcher),
        property!(&RetBlock.ret_statement(), ret_matcher)
      )
    )))
  }
}
