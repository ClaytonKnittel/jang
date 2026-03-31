use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct CallExpression {
  name: Ident,
  parameter_list: Vec<Expression>,
}

impl CallExpression {
  pub fn new(name: Ident, parameter_list: Vec<Expression>) -> Self {
    Self {
      name,
      parameter_list,
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
