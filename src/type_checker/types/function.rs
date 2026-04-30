use crate::type_checker::types::registry::Ty;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType<'a> {
  parameters: Vec<Ty<'a>>,
  return_type: Ty<'a>,
}

impl<'a> FunctionType<'a> {
  pub fn new(parameters: Vec<Ty<'a>>, return_type: Ty<'a>) -> Self {
    FunctionType {
      parameters,
      return_type,
    }
  }

  pub fn parameters(&self) -> &[Ty<'a>] {
    &self.parameters
  }

  pub fn return_type(&self) -> Ty<'a> {
    self.return_type
  }
}

impl<'a> Display for FunctionType<'a> {
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
  use crate::type_checker::types::concrete::{ConcreteType, matchers::concrete_fn_type};
  use googletest::prelude::*;

  pub fn fn_param_types<'a>(
    params: impl Matcher<&'a [Ty<'a>]>,
  ) -> impl Matcher<&'a ConcreteType<'a>> {
    concrete_fn_type(result_of!(&FunctionType::parameters, params))
  }

  pub fn fn_return_type<'a>(ret: impl Matcher<&'a Ty<'a>>) -> impl Matcher<&'a ConcreteType<'a>> {
    concrete_fn_type(pat!(FunctionType {
      return_type: ret,
      ..
    }))
  }
}
