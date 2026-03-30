use std::fmt::Display;

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
}

impl Op {
  pub fn from_char(ch: char) -> Option<Self> {
    match ch {
      '=' => Some(Self::Equal),
      ',' => Some(Self::Comma),
      '(' => Some(Self::OpenParen),
      ')' => Some(Self::CloseParen),
      '-' => Some(Self::Dash),
      '<' => Some(Self::LessThan),
      '>' => Some(Self::GreaterThan),
      ':' => Some(Self::Colon),
      '.' => Some(Self::Dot),
      _ => None,
    }
  }

  fn to_char(self) -> char {
    match self {
      Self::Equal => '=',
      Self::Comma => ',',
      Self::OpenParen => '(',
      Self::CloseParen => ')',
      Self::Dash => '-',
      Self::LessThan => '<',
      Self::GreaterThan => '>',
      Self::Colon => ':',
      Self::Dot => '.',
    }
  }
}

impl Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_char())
  }
}

pub fn is_op(ch: char) -> bool {
  Op::from_char(ch).is_some()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacing {
  /// This operator is followed by either something that isn't an operator, or
  /// whitespace.
  Alone,
  /// This operator is joined to the following operator. Joining happens
  /// whenever two operators are adjacent without separating whitespace,
  /// regardless of whether the two operators form a joined operator in the
  /// grammar.
  Joint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operator {
  op: Op,
  spacing: Spacing,
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
pub mod matchers {
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
        &$crate::parser::token::operator::Spacing::Alone,
      )
    };
  }

  #[macro_export]
  macro_rules! joint_operator {
    ($op:ident) => {
      $crate::parser::token::operator::matchers::operator_matcher(
        &$crate::parser::token::operator::Op::$op,
        &$crate::parser::token::operator::Spacing::Joint,
      )
    };
  }
}
