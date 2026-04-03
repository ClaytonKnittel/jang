use std::fmt::Display;

use crate::parser::{
  ast::{
    binary_expression::BinaryExpression, dot_expression::DotExpression,
    standalone_expression::StandaloneExpression,
  },
  token::{ident::Ident, literal::Literal},
};

#[derive(Clone, Debug)]
pub enum Expression {
  Literal(Literal),
  Ident(Ident),
  BinaryExpression(BinaryExpression),
  DotExpression(DotExpression),
  StandaloneExpression(StandaloneExpression),
}

impl From<Literal> for Expression {
  fn from(value: Literal) -> Self {
    Self::Literal(value)
  }
}

impl From<Ident> for Expression {
  fn from(value: Ident) -> Self {
    Self::Ident(value)
  }
}

impl From<BinaryExpression> for Expression {
  fn from(value: BinaryExpression) -> Self {
    Self::BinaryExpression(value)
  }
}

impl From<DotExpression> for Expression {
  fn from(value: DotExpression) -> Self {
    Self::DotExpression(value)
  }
}

impl<T: Into<StandaloneExpression>> From<T> for Expression {
  fn from(value: T) -> Self {
    Self::StandaloneExpression(value.into())
  }
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(literal) => write!(f, "{literal}"),
      Self::Ident(ident) => write!(f, "{ident}"),
      Self::BinaryExpression(binary_expr) => write!(f, "({binary_expr})"),
      Self::DotExpression(dot_expr) => write!(f, "{dot_expr}"),
      Self::StandaloneExpression(expr) => write!(f, "{expr}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::expression::Expression,
    token::{ident::Ident, literal::Literal},
  };
  use googletest::prelude::*;

  pub fn literal_expression<'a>(
    matcher: impl Matcher<&'a Literal>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::Literal(matcher))
  }

  pub fn ident_expression<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a Expression> {
    pat!(Expression::Ident(matcher))
  }
}
