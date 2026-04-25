use std::fmt::Display;

use crate::type_checker::types::concrete::ConcreteType;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct FunctionType {
  parameters: Vec<ConcreteType>,
  return_type: Box<ConcreteType>,
}

impl FunctionType {
  pub fn new(parameters: Vec<ConcreteType>, return_type: Box<ConcreteType>) -> Self {
    FunctionType {
      parameters,
      return_type,
    }
  }

  pub fn parameters(&self) -> &[ConcreteType] {
    &self.parameters
  }

  pub fn return_type(&self) -> &ConcreteType {
    &self.return_type
  }
}

impl Display for FunctionType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("(")?;
    for (i, param) in self.parameters.iter().enumerate() {
      if i > 0 {
        f.write_str(", ")?;
      }
      write!(f, "{param}")?;
    }
    write!(f, ") -> {}", self.return_type)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::concrete::matchers::concrete_fn_type;
  use googletest::prelude::*;
  use std::ops::Deref;

  pub fn fn_param_types<'a>(
    params: impl Matcher<&'a Vec<ConcreteType>>,
  ) -> impl Matcher<&'a ConcreteType> {
    concrete_fn_type(pat!(FunctionType {
      parameters: params,
      ..
    }))
  }

  pub fn fn_return_type<'a>(ret: impl Matcher<&'a ConcreteType>) -> impl Matcher<&'a ConcreteType> {
    concrete_fn_type(pat!(FunctionType {
      return_type: result_of!(&Box::deref, ret),
      ..
    }))
  }
}
