use std::fmt::Display;

use crate::parser::token::ident::Ident;

#[derive(Clone, Debug)]
pub struct Type(pub Ident);

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{ast::type_expr::Type, token::ident::Ident};
  use googletest::prelude::{Matcher, pat};

  pub fn type_expr_name<'a>(expected: impl Matcher<&'a Ident>) -> impl Matcher<&'a Type> {
    pat!(Type(expected))
  }
}
