use std::fmt::Display;

use crate::parser::token::literal::Literal;

#[derive(Clone, Debug)]
pub struct LiteralExpression {
  literal: Literal,
}

impl LiteralExpression {
  pub fn new(literal: Literal) -> Self {
    Self { literal }
  }

  pub fn literal(&self) -> &Literal {
    &self.literal
  }
}

impl Display for LiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.literal)
  }
}

#[cfg(test)]
pub mod matchers {
  use googletest::prelude::*;

  use crate::parser::{
    ast::{
      expression::{Expression, ExpressionVariant, matchers::expr_variant},
      literal_expression::LiteralExpression,
    },
    token::literal::Literal,
  };

  pub fn literal_expression<'a>(
    matcher: impl Matcher<&'a Literal>,
  ) -> impl Matcher<&'a Expression> {
    expr_variant(pat!(ExpressionVariant::Literal(pat!(LiteralExpression {
      literal: matcher,
    }))))
  }
}
