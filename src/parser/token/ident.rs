#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }

  pub fn name(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::token::{JangToken, ident::Ident};
  use googletest::prelude::*;

  pub fn ident(expected_name: &str) -> impl Matcher<&JangToken> {
    pat!(JangToken::Ident(pat!(Ident(eq(expected_name)))))
  }
}
