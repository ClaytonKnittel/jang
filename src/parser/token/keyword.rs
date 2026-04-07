use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
  Function,
  Let,
  Ret,
  If,
  Else,
  Loop,
  Break,
  Type,
}

impl Keyword {
  pub fn build_from_string(string_val: &str) -> Option<Self> {
    match string_val {
      "fn" => Some(Self::Function),
      "let" => Some(Self::Let),
      "ret" => Some(Self::Ret),
      "if" => Some(Self::If),
      "else" => Some(Self::Else),
      "loop" => Some(Self::Loop),
      "break" => Some(Self::Break),
      "type" => Some(Self::Type),
      _ => None,
    }
  }
}

impl Display for Keyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Function => "fn",
        Self::Let => "let",
        Self::Ret => "ret",
        Self::If => "if",
        Self::Else => "else",
        Self::Loop => "loop",
        Self::Break => "break",
        Self::Type => "type",
      }
    )
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::{JangToken, keyword::Keyword};
  use googletest::prelude::*;

  pub fn keyword_matcher(keyword: &Keyword) -> impl Matcher<&JangToken> {
    pat!(JangToken::Keyword(eq(keyword)))
  }

  #[macro_export]
  macro_rules! keyword {
    ($keyword:ident) => {
      $crate::parser::token::keyword::matchers::keyword_matcher(
        &$crate::parser::token::keyword::Keyword::$keyword,
      )
    };
  }
}
