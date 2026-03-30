use crate::parser::{
  ast::binary_expression::BinaryExpression,
  token::{ident::Ident, literal::Literal},
};

#[derive(Clone, Debug)]
pub enum Expression {
  Literal(Literal),
  Ident(Ident),
  BinaryExpression(BinaryExpression),
}

#[cfg(test)]
pub mod matchers {
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
