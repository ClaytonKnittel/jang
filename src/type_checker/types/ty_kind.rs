use std::fmt::Display;

use crate::type_checker::types::{
  function::FunctionType, primitive::PrimitiveType, strukt::StructType,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TyKind<'ctx> {
  Unit,
  Function(FunctionType<'ctx>),
  Primitive(PrimitiveType),
  Struct(StructType<'ctx>),
}

impl<'ctx> Display for TyKind<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unit => f.write_str("unit"),
      Self::Primitive(p) => write!(f, "{p}"),
      Self::Function(func) => write!(f, "{func}"),
      Self::Struct(strukt) => write!(f, "{strukt}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::{
    registry::{Ty, matchers::ty},
    strukt::StructType,
    ty_kind::TyKind,
  };
  use googletest::prelude::*;

  pub fn concrete_primitive_type<'a>(
    primitive_matcher: impl Matcher<&'a PrimitiveType>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyKind::Primitive(primitive_matcher)))
  }

  pub fn concrete_fn_type<'a>(
    function_matcher: impl Matcher<&'a FunctionType<'a>>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyKind::Function(function_matcher)))
  }

  pub fn unit_type<'a>() -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyKind::Unit))
  }

  pub fn struct_type<'a>(
    struct_matcher: impl Matcher<&'a StructType<'a>>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyKind::Struct(struct_matcher)))
  }
}
