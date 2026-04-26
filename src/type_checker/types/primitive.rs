use std::fmt::Display;

use crate::parser::ast::type_expr::primitive::PrimitiveType as AstPrimitiveType;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum PrimitiveType {
  I32,
  I64,
  F32,
  F64,
  Bool,
}

impl Display for PrimitiveType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = match self {
      Self::I32 => "i32",
      Self::I64 => "i64",
      Self::F32 => "f32",
      Self::F64 => "f64",
      Self::Bool => "bool",
    };
    f.write_str(name)
  }
}

impl From<&AstPrimitiveType> for PrimitiveType {
  fn from(value: &AstPrimitiveType) -> Self {
    match value {
      AstPrimitiveType::Bool => PrimitiveType::Bool,
      AstPrimitiveType::I32 => PrimitiveType::I32,
      AstPrimitiveType::I64 => PrimitiveType::I64,
      AstPrimitiveType::F32 => PrimitiveType::F32,
      AstPrimitiveType::F64 => PrimitiveType::F64,
    }
  }
}

impl PrimitiveType {
  pub fn is_integer(&self) -> bool {
    matches!(self, PrimitiveType::I32 | PrimitiveType::I64)
  }

  pub fn is_float(&self) -> bool {
    matches!(self, PrimitiveType::F32 | PrimitiveType::F64)
  }

  pub fn is_numeric(&self) -> bool {
    self.is_integer() || self.is_float()
  }

  pub fn is_bool(&self) -> bool {
    matches!(self, PrimitiveType::Bool)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::type_checker::types::concrete::{ConcreteType, matchers::concrete_primitive_type};

  use super::*;
  use googletest::prelude::*;

  pub fn i32_type<'a>() -> impl Matcher<&'a ConcreteType> {
    concrete_primitive_type(pat!(PrimitiveType::I32))
  }

  pub fn i64_type<'a>() -> impl Matcher<&'a ConcreteType> {
    concrete_primitive_type(pat!(PrimitiveType::I64))
  }

  pub fn f32_type<'a>() -> impl Matcher<&'a ConcreteType> {
    concrete_primitive_type(pat!(PrimitiveType::F32))
  }

  pub fn f64_type<'a>() -> impl Matcher<&'a ConcreteType> {
    concrete_primitive_type(pat!(PrimitiveType::F64))
  }

  pub fn bool_type<'a>() -> impl Matcher<&'a ConcreteType> {
    concrete_primitive_type(pat!(PrimitiveType::Bool))
  }

  pub fn i32_type_name<'a>() -> impl Matcher<&'a str> {
    eq(PrimitiveType::I32.to_string())
  }

  pub fn i64_type_name<'a>() -> impl Matcher<&'a str> {
    eq(PrimitiveType::I64.to_string())
  }

  pub fn f32_type_name<'a>() -> impl Matcher<&'a str> {
    eq(PrimitiveType::F32.to_string())
  }

  pub fn f64_type_name<'a>() -> impl Matcher<&'a str> {
    eq(PrimitiveType::F64.to_string())
  }

  pub fn bool_type_name<'a>() -> impl Matcher<&'a str> {
    eq(PrimitiveType::Bool.to_string())
  }
}
