use std::{borrow::Borrow, fmt::Display, hash::Hash};

use crate::source_location::SourceSpan;

#[derive(Clone, Debug)]
pub struct Ident {
  name: String,
  span: SourceSpan,
}

impl Ident {
  pub fn new(name: impl Into<String>, span: SourceSpan) -> Self {
    Self {
      name: name.into(),
      span,
    }
  }

  #[cfg(test)]
  pub fn new_isolated(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      span: crate::source_location::SourceLocation::file_start().into(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn span(&self) -> &SourceSpan {
    &self.span
  }
}

impl Borrow<str> for Ident {
  fn borrow(&self) -> &str {
    &self.name
  }
}

impl PartialEq for Ident {
  fn eq(&self, other: &Self) -> bool {
    self.name.eq(&other.name)
  }
}

impl Eq for Ident {}

impl Hash for Ident {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

impl Display for Ident {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::{JangToken, ident::Ident};
  use googletest::prelude::*;

  pub fn ident(expected_name: &str) -> impl Matcher<&Ident> {
    pat!(Ident {
      name: eq(expected_name),
      span: anything(),
    })
  }

  pub fn ident_token(expected_name: &str) -> impl Matcher<&JangToken> {
    pat!(JangToken::Ident(ident(expected_name)))
  }
}
