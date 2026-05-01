use crate::{adt::arena::Arena, type_checker::types::concrete::ConcreteType};

/// Context needed for a type checking pass.
#[derive(Default)]
pub struct TypeCheckerCtx<'ctx> {
  types: Arena<ConcreteType<'ctx>>,
}

impl<'ctx> TypeCheckerCtx<'ctx> {
  pub(super) fn types(&self) -> &Arena<ConcreteType<'ctx>> {
    &self.types
  }
}
