use crate::{adt::arena::Arena, type_checker::types::concrete::TyKind};

/// Context needed for a type checking pass.
#[derive(Default)]
pub struct TypeCheckerCtx<'ctx> {
  types: Arena<TyKind<'ctx>>,
}

impl<'ctx> TypeCheckerCtx<'ctx> {
  pub(super) fn types(&self) -> &Arena<TyKind<'ctx>> {
    &self.types
  }
}
