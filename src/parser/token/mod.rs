pub mod ident;
pub mod keyword;
pub mod literal;
#[cfg(test)]
pub mod test_util;

use crate::parser::token::{ident::Ident, keyword::Keyword, literal::Literal};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JangToken {
  Ident(Ident),
  Keyword(Keyword),
  Literal(Literal),
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
