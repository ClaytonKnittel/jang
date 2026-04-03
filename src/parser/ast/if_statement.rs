use std::fmt::Display;

use crate::parser::ast::{block::Block, expression::Expression};

#[derive(Clone, Debug)]
pub struct IfStatement {
  condition: Box<Expression>,
  body: Box<Block>,
  else_expr: Option<Box<Expression>>,
}

impl IfStatement {
  pub fn new(condition: impl Into<Box<Expression>>, body: impl Into<Box<Block>>) -> Self {
    Self {
      condition: condition.into(),
      body: body.into(),
      else_expr: None,
    }
  }

  pub fn new_with_else(
    condition: impl Into<Box<Expression>>,
    body: impl Into<Box<Block>>,
    else_expr: impl Into<Box<Expression>>,
  ) -> Self {
    Self {
      condition: condition.into(),
      body: body.into(),
      else_expr: Some(else_expr.into()),
    }
  }

  pub fn condition(&self) -> &Expression {
    &self.condition
  }

  pub fn body(&self) -> &Block {
    &self.body
  }

  pub fn else_expr(&self) -> Option<&Expression> {
    self.else_expr.as_deref()
  }
}

impl Display for IfStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "if {} {}", self.condition(), self.body())?;
    if let Some(else_expr) = self.else_expr() {
      write!(f, "{else_expr}")?;
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
      body: result_of!(Box::as_ref, body_matcher),
      else_expr: none()
    })))
  }
}
