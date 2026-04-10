use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumericLiteral {
  Integral(String),
  Float(String),
}

impl NumericLiteral {
  pub fn from_str(string_val: impl Into<String>) -> Self {
    let string_val = string_val.into();
    if string_val.chars().any(|ch| ch == '.') {
      Self::Float(string_val)
    } else {
      Self::Integral(string_val)
    }
  }
}

impl Display for NumericLiteral {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Integral(val) | Self::Float(val) => write!(f, "{val}"),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
  Numeric(NumericLiteral),
}

impl From<NumericLiteral> for Literal {
  fn from(value: NumericLiteral) -> Self {
    Self::Numeric(value)
  }
}

impl Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Numeric(numeric_literal) => write!(f, "{numeric_literal}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::token::{
    JangToken,
    literal::{Literal, NumericLiteral},
  };
  use googletest::prelude::*;

  pub fn integral(expected_value: &str) -> impl Matcher<&Literal> {
    pat!(Literal::Numeric(pat!(NumericLiteral::Integral(eq(
      expected_value
    )))))
  }

  pub fn integral_token(expected_value: &str) -> impl Matcher<&JangToken> {
    pat!(JangToken::Literal(integral(expected_value)))
  }

  pub fn float(expected_value: &str) -> impl Matcher<&Literal> {
    pat!(Literal::Numeric(pat!(NumericLiteral::Float(eq(
      expected_value
    )))))
  }

  pub fn float_token(expected_value: &str) -> impl Matcher<&JangToken> {
    pat!(JangToken::Literal(float(expected_value)))
  }
}
