use std::fmt::Display;

use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct DotExpression {
  base: Box<Expression>,
  member: Ident,
}

impl DotExpression {
  pub fn new(base: impl Into<Box<Expression>>, member: Ident) -> Self {
    Self {
      base: base.into(),
      member,
    }
  }

  pub fn base(&self) -> &Expression {
    &self.base
  }

  pub fn member(&self) -> &Ident {
    &self.member
  }
}

impl Display for DotExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}", self.base, self.member)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{dot_expression::DotExpression, expression::Expression},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn dot_expr_base<'a>(
    base_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::DotExpression(property!(
      &DotExpression.base(),
      base_matcher
    )))
  }

  pub fn dot_expr_member<'a>(
    member_matcher: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::DotExpression(property!(
      &DotExpression.member(),
      member_matcher
    )))
  }
}
