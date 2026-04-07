use std::fmt::Display;

use crate::parser::token::ident::Ident;

#[derive(Clone, Debug)]
pub struct TypeExpression(pub Ident);

impl Display for TypeExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{ast::type_expr::TypeExpression, token::ident::Ident};
  use googletest::prelude::{Matcher, pat};

  pub fn type_expr_name<'a>(expected: impl Matcher<&'a Ident>) -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression(expected))
  }
}
