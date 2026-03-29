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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
  Numeric(NumericLiteral),
}

impl From<NumericLiteral> for Literal {
  fn from(value: NumericLiteral) -> Self {
    Self::Numeric(value)
  }
}
