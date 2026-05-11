use std::fmt::Display;

use crate::type_checker::types::{
  function::FunctionType, primitive::PrimitiveType, strukt::StructType,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TyTerm<'ctx> {
  Unit,
  Function(FunctionType<'ctx>),
  Primitive(PrimitiveType),
  Struct(StructType<'ctx>),
}

impl<'ctx> Display for TyTerm<'ctx> {
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
    ty_term::TyTerm,
  };
  use googletest::prelude::*;

  pub fn concrete_primitive_type<'a>(
    primitive_matcher: impl Matcher<&'a PrimitiveType>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyTerm::Primitive(primitive_matcher)))
  }

  pub fn concrete_fn_type<'a>(
    function_matcher: impl Matcher<&'a FunctionType<'a>>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyTerm::Function(function_matcher)))
  }

  pub fn unit_type<'a>() -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyTerm::Unit))
  }

  pub fn struct_type<'a>(
    struct_matcher: impl Matcher<&'a StructType<'a>>,
  ) -> impl Matcher<&'a Ty<'a>> {
    ty(pat!(TyTerm::Struct(struct_matcher)))
  }
}
