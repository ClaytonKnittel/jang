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
  Mut,
  Unit,
  Bool,
  I32,
  I64,
  F32,
  F64,
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
      "mut" => Some(Self::Mut),
      "unit" => Some(Self::Unit),
      "bool" => Some(Self::Bool),
      "i32" => Some(Self::I32),
      "i64" => Some(Self::I64),
      "f32" => Some(Self::F32),
      "f64" => Some(Self::F64),
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
        Self::Mut => "mut",
        Self::Unit => "unit",
        Self::Bool => "bool",
        Self::I32 => "i32",
        Self::I64 => "i64",
        Self::F32 => "f32",
        Self::F64 => "f64",
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
