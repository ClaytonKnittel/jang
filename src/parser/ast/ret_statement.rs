use std::fmt::Display;

use crate::parser::ast::{block::RetBlock, expression::Expression};

#[derive(Clone, Debug)]
pub struct RetExpression {
  expr: Expression,
}

impl RetExpression {
  pub fn new(expr: Expression) -> Self {
    Self { expr }
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

impl Display for RetExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ret {}", self.expr)
  }
}

#[derive(Clone, Debug)]
pub enum RetStatement {
  Ret(RetExpression),
  Block(Box<RetBlock>),
}

impl From<RetExpression> for RetStatement {
  fn from(value: RetExpression) -> Self {
    Self::Ret(value)
  }
}

impl<T: Into<Box<RetBlock>>> From<T> for RetStatement {
  fn from(value: T) -> Self {
    Self::Block(value.into())
  }
}

impl Display for RetStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Ret(ret_expr) => write!(f, "{ret_expr}"),
      Self::Block(ret_block) => write!(f, "{ret_block}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    expression::Expression,
    ret_statement::{RetExpression, RetStatement},
  };
  use googletest::prelude::*;

  pub fn ret_expression<'a>(
    expected: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a RetStatement> {
    pat!(RetStatement::Ret(pat!(RetExpression { expr: expected })))
  }
}
