use crate::parser::token::literal::Literal;

#[derive(Clone, Debug)]
pub enum Expression {
  Literal(Literal),
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::{ast::expression::Expression, token::literal::Literal};
  use googletest::prelude::*;

  pub fn literal_expression<'a>(
    matcher: impl Matcher<&'a Literal>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::Literal(matcher))
  }
}
