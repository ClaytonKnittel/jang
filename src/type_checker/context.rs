use crate::{adt::arena::Arena, type_checker::types::ty_term::TyTerm};

/// Context needed for a type checking pass.
#[derive(Default)]
pub struct TypeCheckerCtx<'ctx> {
  types: Arena<TyTerm<'ctx>>,
}

impl<'ctx> TypeCheckerCtx<'ctx> {
  pub(super) fn types(&self) -> &Arena<TyTerm<'ctx>> {
    &self.types
  }
}
