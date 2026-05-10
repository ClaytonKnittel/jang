use crate::type_checker::types::ty_kind::TyKind;

/// A type that may be undergoing inference — it can be a concrete compound
/// type whose sub-components may themselves be type variables.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InferredTy<'ctx>(&'ctx InferredTyKind<'ctx>);

impl<'ctx> InferredTy<'ctx> {
  pub(crate) fn new(kind: &'ctx InferredTyKind<'ctx>) -> Self {
    Self(kind)
  }

  pub(crate) fn kind(&self) -> &'ctx InferredTyKind<'ctx> {
    self.0
  }
}

/// The content of an [`InferredTy`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferredTyKind<'ctx> {
  /// A compound type; inner references can be type variables.
  Ty(TyKind<InferredTy<'ctx>>),
  /// A type variable.
  Var(TypeVarId),
}

/// An ID for a type variable.
/// This is an ID of a node in the [`InferenceTable`]'s [`UnionFind`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeVarId(pub(crate) usize);
