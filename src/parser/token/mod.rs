pub mod ident;
pub mod keyword;
pub mod literal;
pub mod operator;

use cknittel_util::from_variants::FromVariants;

use crate::parser::token::{ident::Ident, keyword::Keyword, literal::Literal, operator::Operator};

#[derive(Clone, Debug, PartialEq, Eq, FromVariants)]
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

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::JangToken;
  use googletest::prelude::*;

  pub fn joint<'a>() -> impl Matcher<&'a JangToken> {
    pat!(JangToken::Joint)
  }
}
