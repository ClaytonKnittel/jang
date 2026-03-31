pub mod ident;
pub mod keyword;
pub mod literal;
pub mod operator;

use crate::parser::token::{ident::Ident, keyword::Keyword, literal::Literal, operator::Operator};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JangToken {
  Ident(Ident),
  Keyword(Keyword),
  Literal(Literal),
  Operator(Operator),
  /// Joint tokens are placed between operators which are not separated by
  /// whitespcae, or between an identifier which is immediately followed by
  /// open parenthesis.
  Joint,
}

impl From<Ident> for JangToken {
  fn from(value: Ident) -> Self {
    Self::Ident(value)
  }
}

impl From<Keyword> for JangToken {
  fn from(value: Keyword) -> Self {
    Self::Keyword(value)
  }
}

impl<L: Into<Literal>> From<L> for JangToken {
  fn from(value: L) -> Self {
    Self::Literal(value.into())
  }
}

impl From<Operator> for JangToken {
  fn from(value: Operator) -> Self {
    Self::Operator(value)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::JangToken;
  use googletest::prelude::*;

  pub fn joint<'a>() -> impl Matcher<&'a JangToken> {
    pat!(JangToken::Joint)
  }
}
