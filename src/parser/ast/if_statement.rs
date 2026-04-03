use std::fmt::Display;

use crate::parser::ast::{block::Block, expression::Expression};

#[derive(Clone, Debug)]
pub struct IfStatement {
  condition: Box<Expression>,
  body: Block,
  else_block: Option<Block>,
}

impl IfStatement {
  pub fn new(condition: impl Into<Box<Expression>>, body: Block) -> Self {
    Self {
      condition: condition.into(),
      body,
      else_block: None,
    }
  }

  pub fn new_with_else(
    condition: impl Into<Box<Expression>>,
    body: Block,
    else_block: Block,
  ) -> Self {
    Self {
      condition: condition.into(),
      body,
      else_block: Some(else_block),
    }
  }

  pub fn condition(&self) -> &Expression {
    &self.condition
  }

  pub fn body(&self) -> &Block {
    &self.body
  }

  pub fn else_block(&self) -> Option<&Block> {
    self.else_block.as_ref()
  }
}

impl Display for IfStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "if {} {}", self.condition(), self.body())?;
    if let Some(else_block) = self.else_block() {
      write!(f, "{else_block}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    block::Block, expression::Expression, if_statement::IfStatement, statement::NonRetStatement,
  };
  use googletest::prelude::*;

  pub fn if_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::IfStatement(pat!(IfStatement {
      condition: result_of!(Box::as_ref, cond_matcher),
      body: body_matcher,
      else_block: none(),
    })))
  }

  pub fn if_else_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
    else_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::IfStatement(pat!(IfStatement {
      condition: result_of!(Box::as_ref, cond_matcher),
      body: body_matcher,
      else_block: some(else_matcher),
    })))
  }
}
