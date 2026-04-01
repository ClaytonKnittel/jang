use std::fmt::Display;

use crate::parser::ast::{block::NonRetBlock, expression::Expression, let_statement::LetStatement};

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

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{expression::Expression, statement::NonRetStatement};
  use googletest::prelude::*;

  pub fn expr_stmt<'a>(
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::Expression(expr_matcher))
  }
}
