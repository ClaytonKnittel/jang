use std::fmt::Display;

use crate::parser::ast::{
  block::{NonRetBlock, RetBlock},
  expression::Expression,
  let_statement::LetStatement,
};

#[derive(Clone, Debug)]
pub enum NonRetStatement {
  Let(LetStatement),
  Expression(Expression),
  Block(NonRetBlock),
}

impl From<LetStatement> for NonRetStatement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
  }
}

impl From<Expression> for NonRetStatement {
  fn from(value: Expression) -> Self {
    Self::Expression(value)
  }
}

impl From<NonRetBlock> for NonRetStatement {
  fn from(value: NonRetBlock) -> Self {
    Self::Block(value)
  }
}

impl Display for NonRetStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Let(let_stmt) => write!(f, "{let_stmt}"),
      Self::Expression(expr) => write!(f, "{expr}"),
      Self::Block(block) => write!(f, "{block}"),
    }
  }
}

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
    statement::{NonRetStatement, RetExpression, RetStatement},
  };
  use googletest::prelude::*;

  pub fn expr_stmt<'a>(
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::Expression(expr_matcher))
  }

  pub fn ret_expression<'a>(
    expected: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a RetStatement> {
    pat!(RetStatement::Ret(pat!(RetExpression { expr: expected })))
  }
}
