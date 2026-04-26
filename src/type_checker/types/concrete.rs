use crate::type_checker::types::{
  function::FunctionType,
  primitive::PrimitiveType,
  registry::{DisplayType, TypeRegistry},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ConcreteType {
  Unit,
  Function(FunctionType),
  Primitive(PrimitiveType),
}

impl DisplayType for ConcreteType {
  fn fmt_type(&self, f: &mut std::fmt::Formatter<'_>, r: &TypeRegistry) -> std::fmt::Result {
    match self {
      Self::Unit => f.write_str("unit"),
      Self::Primitive(p) => write!(f, "{p}"),
      Self::Function(func) => func.fmt_type(f, r),
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
