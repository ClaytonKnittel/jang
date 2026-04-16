use std::{fmt::Display, hint::unreachable_unchecked};

use cknittel_util::peekable_stream::ItemProxy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
  /// '='
  Equal,
  /// ','
  Comma,
  /// '('
  OpenParen,
  /// ')'
  CloseParen,
  /// '{'
  OpenBracket,
  /// '}'
  CloseBracket,
  /// '-'
  Dash,
  /// '<'
  LessThan,
  /// '>'
  GreaterThan,
  /// ':'
  Colon,
  /// '.'
  Dot,
  /// '+'
  Plus,
  /// '*'
  Star,
  /// '/'
  Slash,
  /// '%'
  Percent,
  /// '|'
  Bar,
  /// '!'
  Bang,
  /// '&'
  Ampersand,

  // Multi-token operators:
  /// ">="
  GreaterOrEqual,
  /// "<="
  LessOrEqual,
  /// "=="
  DoubleEqual,
  /// "!="
  NotEqual,
  /// "&&"
  LogicalAnd,
  /// "||"
  LogicalOr,
  /// "->"
  RightArrow,
  /// "=>"
  ThiqqRightArrow,
}

impl Op {
  pub fn from_char_with_peek<I: Iterator<Item = Result<char, E>>, E>(
    ch: char,
    peek: Option<ItemProxy<'_, I>>,
  ) -> Self {
    macro_rules! match_next {
      (_ => $default:expr) => {
        Some($default)
      };

      (
        $($next_char:literal => $op:expr,)+
        _ => $default:expr
      ) => {
        match peek {
          Some(peek) => {
            match *peek {
              $(Ok($next_char) => {
                unsafe { peek.take().unwrap_unchecked() };
                $op
              })+
              _ => $default,
            }
          }
          _ => $default
        }
      };
    }

    match ch {
      '=' => match_next!(
        '=' => Self::DoubleEqual,
        '>' => Self::ThiqqRightArrow,
        _ => Self::Equal
      ),
      ',' => Self::Comma,
      '(' => Self::OpenParen,
      ')' => Self::CloseParen,
      '{' => Self::OpenBracket,
      '}' => Self::CloseBracket,
      '-' => match_next!(
        '>' => Self::RightArrow,
        _ => Self::Dash
      ),
      '<' => match_next!(
        '=' => Self::LessOrEqual,
        _ => Self::LessThan
      ),
      '>' => match_next!(
        '=' => Self::GreaterOrEqual,
        _ => Self::GreaterThan
      ),
      ':' => Self::Colon,
      '.' => Self::Dot,
      '+' => Self::Plus,
      '*' => Self::Star,
      '/' => Self::Slash,
      '%' => Self::Percent,
      '|' => match_next!(
        '|' => Self::LogicalOr,
        _ => Self::Bar
      ),
      '!' => match_next!(
        '=' => Self::NotEqual,
        _ => Self::Bang
      ),
      '&' => match_next!(
        '&' => Self::LogicalAnd,
        _ => Self::Ampersand
      ),
      _ => unsafe { unreachable_unchecked() },
    }
  }

  /// Returns true if this operator can join the next character.
  pub fn can_join(&self, other_op: char) -> bool {
    match self {
      Self::CloseParen => other_op == '(',
      _ => false,
    }
  }
}

impl Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Equal => "=",
        Self::Comma => ",",
        Self::OpenParen => "(",
        Self::CloseParen => ")",
        Self::OpenBracket => "{",
        Self::CloseBracket => "}",
        Self::Dash => "-",
        Self::LessThan => "<",
        Self::GreaterThan => ">",
        Self::Colon => ":",
        Self::Dot => ".",
        Self::Plus => "+",
        Self::Star => "*",
        Self::Slash => "/",
        Self::Percent => "%",
        Self::Bar => "|",
        Self::Bang => "!",
        Self::Ampersand => "&",
        Self::GreaterOrEqual => ">=",
        Self::LessOrEqual => "<=",
        Self::DoubleEqual => "==",
        Self::NotEqual => "!=",
        Self::LogicalAnd => "&&",
        Self::LogicalOr => "||",
        Self::RightArrow => "->",
        Self::ThiqqRightArrow => "=>",
      }
    )
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operator {
  pub op: Op,
}

impl Operator {
  pub fn new(op: Op) -> Operator {
    Self { op }
  }

  pub fn op(&self) -> Op {
    self.op
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::{
    JangToken,
    operator::{Op, Operator},
  };
  use googletest::prelude::*;

  pub fn operator_matcher(op: &Op) -> impl Matcher<&JangToken> {
    pat!(JangToken::Operator(pat!(Operator { op: op })))
  }

  #[macro_export]
  macro_rules! operator {
    ($op:ident) => {
      $crate::parser::token::operator::matchers::operator_matcher(
        &$crate::parser::token::operator::Op::$op,
      )
    };
  }
}
