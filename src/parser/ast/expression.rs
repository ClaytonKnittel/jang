use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::{
  ast::{
    binary_expression::BinaryExpression, call_expression::CallExpression,
    dot_expression::DotExpression,
  },
  token::{ident::Ident, literal::Literal},
};

#[derive(Clone, Debug, FromVariants)]
pub enum Expression {
  Literal(Literal),
  Ident(Ident),
  BinaryExpression(BinaryExpression),
  CallExpression(CallExpression),
  DotExpression(DotExpression),
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(literal) => write!(f, "{literal}"),
      Self::Ident(ident) => write!(f, "{ident}"),
      Self::BinaryExpression(binary_expr) => write!(f, "({binary_expr})"),
      Self::CallExpression(call_expr) => write!(f, "{call_expr}"),
      Self::DotExpression(dot_expr) => write!(f, "{dot_expr}"),
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
