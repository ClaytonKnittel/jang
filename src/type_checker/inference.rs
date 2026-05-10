use std::{error::Error, fmt::Display, ops::Deref};

use cknittel_util::{
  from_variants::FromVariants,
  union_find::{UnionFind, UnionFindData, UnionFindMergeError},
};

use crate::type_checker::{
  error::{TypeCheckerError, TypeCheckerResult},
  types::{
    concrete::TyKind,
    primitive::PrimitiveType,
    registry::{Ty, TypeRegistry},
  },
};

/// A handle for a Ty that may be undergoing inference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InferredTy<'ctx>(InferredTyKind<'ctx>);

/// The content of a type that may be undergoing inference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromVariants)]
enum InferredTyKind<'ctx> {
  Ty(Ty<'ctx>),
  Var(TypeVarId),
}

impl<'ctx> From<Ty<'ctx>> for InferredTy<'ctx> {
  fn from(ty: Ty<'ctx>) -> Self {
    Self(ty.into())
  }
}

#[derive(Default)]
pub struct InferenceTable<'ctx> {
  vars: UnionFind<TypeVar<'ctx>>,
}

impl<'ctx> InferenceTable<'ctx> {
  /// Creates a new integral type variable.
  pub fn new_integral_var(&mut self) -> InferredTy<'ctx> {
    self.new_var(Constraint::Integral)
  }

  /// Creates a new floating type variable.
  pub fn new_floating_var(&mut self) -> InferredTy<'ctx> {
    self.new_var(Constraint::Floating)
  }

  fn new_var(&mut self, constraint: Constraint) -> InferredTy<'ctx> {
    InferredTy(TypeVarId(self.vars.add_set(TypeVar::Unbound(constraint)).id()).into())
  }

  pub fn resolve(&self, ty: InferredTy<'ctx>, types: &TypeRegistry<'ctx>) -> Ty<'ctx> {
    match ty.0 {
      InferredTyKind::Ty(ty) => ty,
      InferredTyKind::Var(id) => match self.type_var(id) {
        TypeVar::Bound(ty) => ty,
        TypeVar::Unbound(constraint) => constraint.default_type(types),
      },
    }
  }

  pub fn unify(
    &mut self,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
    types: &TypeRegistry<'ctx>,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    match (expected.0, actual.0) {
      // If both are non-variadic, they must be equal.
      (InferredTyKind::Ty(a), InferredTyKind::Ty(b)) => {
        if a == b {
          Ok(a.into())
        } else {
          Err(self.mismatch_error(expected, actual, types))
        }
      }

      // If one type is variadic, it must conform to the concrete type.
      (InferredTyKind::Ty(ty), InferredTyKind::Var(var_id))
      | (InferredTyKind::Var(var_id), InferredTyKind::Ty(ty)) => self
        .bind_var(var_id, ty)
        .map(|_| ty.into())
        .map_err(|_| self.mismatch_error(expected, actual, types)),

      // If both are type variables, they must unify.
      (InferredTyKind::Var(expected_id), InferredTyKind::Var(actual_id)) => self
        .vars
        .try_union(expected_id.0, actual_id.0)
        .map(|root| InferredTy(TypeVarId(root.id()).into()))
        .map_err(|_| self.mismatch_error(expected, actual, types)),
    }
  }

  pub fn check_requirement(
    &self,
    ty: InferredTy<'ctx>,
    requirement: TypeClass,
    types: &TypeRegistry<'ctx>,
  ) -> TypeCheckerResult<'ctx> {
    self
      .type_matches_requirement(ty, requirement)
      .then_some(())
      .ok_or_else(|| TypeCheckerError::TypeClassMismatch {
        expected: requirement,
        actual: self.resolve(ty, types),
      })
  }

  fn type_matches_requirement(&self, ty: InferredTy<'ctx>, requirement: TypeClass) -> bool {
    match ty.0 {
      InferredTyKind::Ty(ty) => requirement.accepts_ty(ty),
      InferredTyKind::Var(id) => match self.type_var(id) {
        TypeVar::Bound(ty) => requirement.accepts_ty(ty),
        TypeVar::Unbound(constraint) => requirement.accepts_constraint(constraint),
      },
    }
  }

  fn mismatch_error(
    &self,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
    types: &TypeRegistry<'ctx>,
  ) -> TypeCheckerError<'ctx> {
    TypeCheckerError::TypeMismatch {
      expected: self.resolve(expected, types),
      actual: self.resolve(actual, types),
    }
  }

  fn bind_var(&mut self, id: TypeVarId, ty: Ty<'ctx>) -> Result<(), TypeVarMergeError> {
    self
      .vars
      .find_mut(id.0)
      .data_mut()
      .merge(TypeVar::Bound(ty))
      .map_err(|_| TypeVarMergeError)
  }

  fn type_var(&self, id: TypeVarId) -> TypeVar<'ctx> {
    *self.vars.const_find(id.0).data()
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeClass {
  Numeric,
  Eq,
}

impl Display for TypeClass {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      TypeClass::Numeric => "numeric",
      TypeClass::Eq => "equality compatible",
    })
  }
}

impl TypeClass {
  /// Whether `ty` is within this type class.
  fn accepts_ty<'ctx>(&self, ty: Ty<'ctx>) -> bool {
    use PrimitiveType::*;
    match self {
      Self::Numeric => matches!(ty.deref(), TyKind::Primitive(I32 | I64 | F32 | F64)),
      Self::Eq => matches!(ty.deref(), TyKind::Primitive(I32 | I64 | Bool)),
    }
  }

  /// Whether `constraint` is within this type class.
  fn accepts_constraint(&self, constraint: Constraint) -> bool {
    use Constraint::*;
    match self {
      Self::Numeric => matches!(constraint, Integral | Floating),
      Self::Eq => matches!(constraint, Integral),
    }
  }
}

/// An ID for a type variable.
/// This is an ID of a node in the [`InferenceTable`]'s [`UnionFind`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TypeVarId(usize);

/// A variable for a type that may change over time during inference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TypeVar<'ctx> {
  Bound(Ty<'ctx>),
  Unbound(Constraint),
}

impl<'ctx> TypeVar<'ctx> {
  /// Whether `self` can be narrowed to `other`.
  fn narrows_to(&self, other: &Self) -> bool {
    use Constraint::*;
    use PrimitiveType::*;
    use TyKind::*;
    use TypeVar::*;

    let (Unbound(this), Bound(other)) = (self, other) else {
      return false;
    };

    matches!(
      (this, other.deref()),
      (Floating, Primitive(F32 | F64)) | (Integral, Primitive(I32 | I64))
    )
  }
}

/// A constraint for a type variable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Constraint {
  Integral,
  Floating,
}

impl Constraint {
  fn default_type<'ctx>(&self, types: &TypeRegistry<'ctx>) -> Ty<'ctx> {
    match self {
      Self::Integral => types.primitive_type(PrimitiveType::I32),
      Self::Floating => types.primitive_type(PrimitiveType::F32),
    }
  }
}

/// Type variables could not be merged.
/// This exists solely so we may implement an error type for UnionFindMergeError,
/// the error will always be replaced before reaching the user.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TypeVarMergeError;
impl Error for TypeVarMergeError {}
impl Display for TypeVarMergeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("incompatible type variables")
  }
}

impl<'ctx> UnionFindData for TypeVar<'ctx> {
  type Error = TypeVarMergeError;

  fn merge(&mut self, other: Self) -> Result<(), UnionFindMergeError<Self, TypeVarMergeError>> {
    if self == &other || other.narrows_to(self) {
      Ok(())
    } else if self.narrows_to(&other) {
      *self = other;
      Ok(())
    } else {
      Err(UnionFindMergeError::new(TypeVarMergeError, other))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use googletest::prelude::*;

  use crate::type_checker::{
    context::TypeCheckerCtx,
    error::matchers::type_mismatch_error,
    types::primitive::matchers::{bool_type, f32_type, f64_type, i32_type, i64_type},
  };

  #[gtest]
  fn unresolved_numeric_vars_resolve_to_default_types() {
    let ctx = TypeCheckerCtx::default();
    let types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::default();

    let integral = inference.new_integral_var();
    let floating = inference.new_floating_var();

    expect_that!(&inference.resolve(integral, &types), i32_type());
    expect_that!(&inference.resolve(floating, &types), f32_type());
  }

  #[gtest]
  fn unify_works_across_multiple_variables() {
    let ctx = TypeCheckerCtx::default();
    let types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::default();

    let var0 = inference.new_integral_var();
    let var1 = inference.new_integral_var();
    let i64_ty = InferredTy::from(types.primitive_type(PrimitiveType::I64));
    inference.unify(var0, var1, &types).unwrap();
    inference.unify(var1, i64_ty, &types).unwrap();

    expect_that!(&inference.resolve(var0, &types), i64_type());
  }

  #[gtest]
  fn unify_rejects_incompatible_bound_types() {
    let ctx = TypeCheckerCtx::default();
    let types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::default();

    let i32 = InferredTy::from(types.primitive_type(PrimitiveType::I32));
    let bool = InferredTy::from(types.primitive_type(PrimitiveType::Bool));

    expect_that!(
      inference.unify(i32, bool, &types),
      err(type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn unify_rejects_incompatible_variables() {
    let ctx = TypeCheckerCtx::default();
    let types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::default();

    let integral_var = inference.new_integral_var();
    let floating_var = inference.new_floating_var();

    expect_that!(
      inference.unify(integral_var, floating_var, &types),
      err(type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn unify_rejects_incompatible_numeric_constraint() {
    let ctx = TypeCheckerCtx::default();
    let types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::default();

    let integral = inference.new_integral_var();
    let f64 = InferredTy::from(types.primitive_type(PrimitiveType::F64));

    expect_that!(
      inference.unify(integral, f64, &types),
      err(type_mismatch_error(i32_type(), f64_type()))
    );
  }
}
