use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
  Bool,
  I32,
  I64,
  F32,
  F64,
}

impl Display for PrimitiveType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Bool => "bool",
        Self::I32 => "i32",
        Self::I64 => "i64",
        Self::F32 => "f32",
        Self::F64 => "f64",
      }
    )
  }
}
