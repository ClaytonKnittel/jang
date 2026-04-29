use crate::type_checker::types::registry::{DisplayType, TypeRegistry, TypeId};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType {
  parameters: Vec<TypeId>,
  return_type: TypeId,
}

impl FunctionType {
  pub fn new(parameters: Vec<TypeId>, return_type: TypeId) -> Self {
    FunctionType {
      parameters,
      return_type,
    }
  }

  pub fn parameters(&self) -> &[TypeId] {
    &self.parameters
  }

  pub fn return_type(&self) -> TypeId {
    self.return_type
  }
}

impl DisplayType for FunctionType {
  fn fmt_type(&self, f: &mut std::fmt::Formatter<'_>, r: &TypeRegistry) -> std::fmt::Result {
    f.write_str("(")?;
    for (i, param) in self.parameters.iter().enumerate() {
      if i > 0 {
        f.write_str(", ")?;
      }
      param.fmt_type(f, r)?;
    }
    write!(f, ") -> ")?;
    self.return_type.fmt_type(f, r)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::concrete::{ConcreteType, matchers::concrete_fn_type};
  use googletest::prelude::*;

  pub fn fn_param_types<'a>(
    params: impl Matcher<&'a [TypeId]>,
  ) -> impl Matcher<&'a ConcreteType> {
    concrete_fn_type(result_of!(&FunctionType::parameters, params))
  }

  pub fn fn_return_type<'a>(
    ret: impl Matcher<&'a TypeId>,
  ) -> impl Matcher<&'a ConcreteType> {
    concrete_fn_type(pat!(FunctionType {
      return_type: ret,
      ..
    }))
  }
}
