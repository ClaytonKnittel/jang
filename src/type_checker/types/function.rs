use std::fmt::Display;

use itertools::Itertools;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType<Ty> {
  parameters: Vec<Ty>,
  return_type: Ty,
}

impl<Ty> FunctionType<Ty> {
  pub fn new(parameters: Vec<Ty>, return_type: Ty) -> Self {
    FunctionType {
      parameters,
      return_type,
    }
  }

  pub fn parameters(&self) -> &[Ty] {
    &self.parameters
  }
}

impl<Ty: Copy> FunctionType<Ty> {
  pub fn return_type(&self) -> Ty {
    self.return_type
  }
}

impl<Ty: Display> Display for FunctionType<Ty> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({}) -> {}",
      self.parameters.iter().format(", "),
      self.return_type
    )
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::{registry::Ty, ty_kind::matchers::concrete_fn_type};
  use googletest::prelude::*;

  pub fn fn_param_types<'a>(params: impl Matcher<&'a [Ty<'a>]>) -> impl Matcher<&'a Ty<'a>> {
    concrete_fn_type(result_of!(&FunctionType::parameters, params))
  }

  pub fn fn_return_type<'a>(ret: impl Matcher<&'a Ty<'a>>) -> impl Matcher<&'a Ty<'a>> {
    concrete_fn_type(pat!(FunctionType {
      return_type: ret,
      ..
    }))
  }
}
