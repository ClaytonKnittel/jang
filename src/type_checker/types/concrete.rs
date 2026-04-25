use std::fmt::Display;

use crate::type_checker::types::{function::FunctionType, primitive::PrimitiveType};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum ConcreteType {
  Unit,
  Function(FunctionType),
  Primitive(PrimitiveType),
}

impl Display for ConcreteType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unit => f.write_str("unit"),
      Self::Primitive(p) => write!(f, "{p}"),
      Self::Function(func) => write!(f, "{func}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::concrete::ConcreteType;
  use googletest::prelude::*;

  pub fn concrete_primitive_type<'a>(
    primitive_matcher: impl Matcher<&'a PrimitiveType>,
  ) -> impl Matcher<&'a ConcreteType> {
    pat!(ConcreteType::Primitive(primitive_matcher))
  }

  pub fn concrete_fn_type<'a>(
    function_matcher: impl Matcher<&'a FunctionType>,
  ) -> impl Matcher<&'a ConcreteType> {
    pat!(ConcreteType::Function(function_matcher))
  }

  pub fn unit_type<'a>() -> impl Matcher<&'a ConcreteType> {
    pat!(ConcreteType::Unit)
  }
}
