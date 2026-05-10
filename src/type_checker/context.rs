use crate::{
  adt::arena::Arena,
  type_checker::types::{inferred_ty::InferredTyKind, registry::Ty, ty_kind::TyKind},
};

/// Context needed for a type checking pass.
#[derive(Default)]
pub struct TypeCheckerCtx<'ctx> {
  ty_kinds: Arena<TyKind<Ty<'ctx>>>,
  inferred_ty_kinds: Arena<InferredTyKind<'ctx>>,
}

impl<'ctx> TypeCheckerCtx<'ctx> {
  pub(super) fn types(&self) -> &Arena<TyKind<Ty<'ctx>>> {
    &self.ty_kinds
  }

  pub(super) fn inferred_kinds(&self) -> &Arena<InferredTyKind<'ctx>> {
    &self.inferred_ty_kinds
  }
}
