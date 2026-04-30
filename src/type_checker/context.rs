use crate::{adt::arena::Arena, type_checker::types::concrete::ConcreteType};

/// Context needed for the type checking process.
#[derive(Default)]
pub struct TypeCheckerCtx<'a> {
  types: Arena<ConcreteType<'a>>,
}

impl<'a> TypeCheckerCtx<'a> {
  pub(super) fn types(&self) -> &Arena<ConcreteType<'a>> {
    &self.types
  }
}
