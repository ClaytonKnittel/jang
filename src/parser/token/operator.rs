use std::fmt::Display;

use crate::parser::token::spacing::Spacing;

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
}

impl Op {
  pub fn from_char(ch: char) -> Option<Self> {
    match ch {
      '=' => Some(Self::Equal),
      ',' => Some(Self::Comma),
      '(' => Some(Self::OpenParen),
      ')' => Some(Self::CloseParen),
      '{' => Some(Self::OpenBracket),
      '}' => Some(Self::CloseBracket),
      '-' => Some(Self::Dash),
      '<' => Some(Self::LessThan),
      '>' => Some(Self::GreaterThan),
      ':' => Some(Self::Colon),
      '.' => Some(Self::Dot),
      '+' => Some(Self::Plus),
      '*' => Some(Self::Star),
      '/' => Some(Self::Slash),
      '%' => Some(Self::Percent),
      _ => None,
    }
  }

  fn to_char(self) -> char {
    match self {
      Self::Equal => '=',
      Self::Comma => ',',
      Self::OpenParen => '(',
      Self::CloseParen => ')',
      Self::OpenBracket => '{',
      Self::CloseBracket => '}',
      Self::Dash => '-',
      Self::LessThan => '<',
      Self::GreaterThan => '>',
      Self::Colon => ':',
      Self::Dot => '.',
      Self::Plus => '+',
      Self::Star => '*',
      Self::Slash => '/',
      Self::Percent => '%',
    }
  }
}

impl Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_char())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operator {
  pub op: Op,
  pub spacing: Spacing,
}

impl Operator {
  pub fn new(op: Op, spacing: Spacing) -> Operator {
    Self { op, spacing }
  }

  pub fn op(&self) -> Op {
    self.op
  }

  pub fn spacing(&self) -> Spacing {
    self.spacing
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::{
    JangToken,
    operator::{Op, Operator, Spacing},
  };
  use googletest::prelude::*;

  pub fn operator_matcher<'a>(op: &'a Op, spacing: &'a Spacing) -> impl Matcher<&'a JangToken> {
    pat!(JangToken::Operator(pat!(Operator {
      op: op,
      spacing: spacing
    })))
  }

  #[macro_export]
  macro_rules! operator {
    ($op:ident) => {
      $crate::parser::token::operator::matchers::operator_matcher(
        &$crate::parser::token::operator::Op::$op,
        &$crate::parser::token::spacing::Spacing::Alone,
      )
    };
  }

  #[macro_export]
  macro_rules! joint_operator {
    ($op:ident) => {
      $crate::parser::token::operator::matchers::operator_matcher(
        &$crate::parser::token::operator::Op::$op,
        &$crate::parser::token::spacing::Spacing::Joint,
      )
    };
  }
}
