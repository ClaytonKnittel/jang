use crate::type_checker::types::registry::Ty;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType<'ctx> {
  parameters: Vec<Ty<'ctx>>,
  return_type: Ty<'ctx>,
}

impl<'ctx> FunctionType<'ctx> {
  pub fn new(parameters: Vec<Ty<'ctx>>, return_type: Ty<'ctx>) -> Self {
    FunctionType {
      parameters,
      return_type,
    }
  }

  pub fn parameters(&self) -> &[Ty<'ctx>] {
    &self.parameters
  }

  pub fn return_type(&self) -> Ty<'ctx> {
    self.return_type
  }
}

impl<'ctx> Display for FunctionType<'ctx> {
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
  use crate::type_checker::types::ty_term::matchers::concrete_fn_type;
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
