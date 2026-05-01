use std::fmt::Display;

use crate::type_checker::types::{function::FunctionType, primitive::PrimitiveType};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ConcreteType<'ctx> {
  Unit,
  Function(FunctionType<'ctx>),
  Primitive(PrimitiveType),
}

impl<'ctx> Display for ConcreteType<'ctx> {
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
  use crate::type_checker::types::{
    concrete::ConcreteType,
    registry::{Ty, matchers::ty},
  };
  use googletest::prelude::*;

  pub fn concrete_primitive_type<'a>(
    primitive_matcher: impl Matcher<&'a PrimitiveType>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(ConcreteType::Primitive(primitive_matcher)))
  }

  pub fn concrete_fn_type<'a>(
    function_matcher: impl Matcher<&'a FunctionType<'a>>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(ConcreteType::Function(function_matcher)))
  }

  pub fn unit_type<'a>() -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(ConcreteType::Unit))
  }
}
