use std::{error::Error, fmt::Display, ops::Deref};

use cknittel_util::union_find::{UnionFind, UnionFindData, UnionFindMergeError};

use crate::type_checker::{
  context::TypeCheckerCtx,
  error::{TypeCheckerError, TypeCheckerResult},
  types::{
    function::FunctionType,
    inferred_ty::{InferredTy, InferredTyKind, TypeVarId},
    primitive::PrimitiveType,
    registry::{Ty, TypeRegistry},
    strukt::{StructField, StructType},
    ty_kind::TyKind,
  },
};

pub struct InferenceTable<'ctx> {
  vars: UnionFind<TypeVar<'ctx>>,
  ctx: &'ctx TypeCheckerCtx<'ctx>,
}

impl<'ctx> InferenceTable<'ctx> {
  pub fn new(ctx: &'ctx TypeCheckerCtx<'ctx>) -> Self {
    Self {
      vars: UnionFind::default(),
      ctx,
    }
  }

  /// Creates a new integral type variable.
  pub fn new_integral_var(&mut self) -> InferredTy<'ctx> {
    self.new_var(Constraint::Integral)
  }

  /// Creates a new floating type variable.
  pub fn new_floating_var(&mut self) -> InferredTy<'ctx> {
    self.new_var(Constraint::Floating)
  }

  fn new_var(&mut self, constraint: Constraint) -> InferredTy<'ctx> {
    let id = TypeVarId(self.vars.add_set(TypeVar::Unbound(constraint)).id());
    InferredTy::new(self.ctx.inferred_kinds().alloc(InferredTyKind::Var(id)))
  }

  /// Lift a concrete `Ty<'ctx>` into an `InferredTy<'ctx>`. Costs one arena allocation.
  pub fn lift_ty(&self, ty: Ty<'ctx>) -> InferredTy<'ctx> {
    InferredTy::new(self.ctx.inferred_kinds().alloc(InferredTyKind::Ty(ty)))
  }

  /// Shallowly convert a `TyKind<Ty>` into a `TyKind<InferredTy>` by lifting each child `Ty`.
  fn lift_kind(&self, kind: &TyKind<Ty<'ctx>>) -> TyKind<InferredTy<'ctx>> {
    match kind {
      TyKind::Unit => TyKind::Unit,
      TyKind::Primitive(p) => TyKind::Primitive(*p),
      TyKind::Function(f) => TyKind::Function(FunctionType::new(
        f.parameters().iter().copied().map(|p| self.lift_ty(p)).collect(),
        self.lift_ty(f.return_type()),
      )),
      TyKind::Struct(s) => TyKind::Struct(StructType::new(
        s.fields().iter().map(|f| StructField::new(f.name().clone(), self.lift_ty(f.ty()))),
      )),
    }
  }

  pub fn resolve(&self, ty: InferredTy<'ctx>, types: &mut TypeRegistry<'ctx>) -> Ty<'ctx> {
    match ty.kind() {
      InferredTyKind::Ty(ty) => *ty,
      InferredTyKind::Compound(kind) => self.resolve_kind(kind, types),
      InferredTyKind::Var(id) => match self.type_var(*id) {
        TypeVar::Bound(ty) => ty,
        TypeVar::Unbound(constraint) => constraint.default_type(types),
      },
    }
  }

  fn resolve_kind(
    &self,
    kind: &TyKind<InferredTy<'ctx>>,
    types: &mut TypeRegistry<'ctx>,
  ) -> Ty<'ctx> {
    match kind {
      TyKind::Unit => types.unit_type(),
      TyKind::Primitive(p) => types.primitive_type(*p),
      TyKind::Function(f) => {
        let params: Vec<Ty<'ctx>> = f
          .parameters()
          .iter()
          .copied()
          .map(|p| self.resolve(p, types))
          .collect();
        let ret = self.resolve(f.return_type(), types);
        types.function_type(params, ret)
      }
      TyKind::Struct(s) => {
        let fields: Vec<StructField<Ty<'ctx>>> = s
          .fields()
          .iter()
          .map(|f| StructField::new(f.name().clone(), self.resolve(f.ty(), types)))
          .collect();
        types.struct_type(fields)
      }
    }
  }

  pub fn unify(
    &mut self,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
    types: &mut TypeRegistry<'ctx>,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    match (expected.kind(), actual.kind()) {
      // Fast path: both fully concrete — pointer equality suffices.
      (InferredTyKind::Ty(a), InferredTyKind::Ty(b)) => {
        if a == b {
          Ok(expected)
        } else {
          Err(self.mismatch_error(expected, actual, types))
        }
      }

      // Both compound inferred — recurse into sub-components.
      // k_e / k_a are 'ctx arena refs and don't borrow self.
      (InferredTyKind::Compound(k_e), InferredTyKind::Compound(k_a)) => {
        let (k_e, k_a): (&TyKind<InferredTy<'ctx>>, &TyKind<InferredTy<'ctx>>) = (k_e, k_a);
        self.unify_kinds(k_e, k_a, expected, actual, types)
      }

      // One concrete, one compound — lift the concrete side and recurse into sub-components.
      (InferredTyKind::Ty(ty), InferredTyKind::Compound(k)) => {
        let lifted = self.lift_kind(ty.deref());
        self.unify_kinds(&lifted, k, expected, actual, types)
      }
      (InferredTyKind::Compound(k), InferredTyKind::Ty(ty)) => {
        let lifted = self.lift_kind(ty.deref());
        self.unify_kinds(k, &lifted, expected, actual, types)
      }

      // Var vs var — union-find merge.
      (InferredTyKind::Var(e_id), InferredTyKind::Var(a_id)) => {
        let (e, a) = (*e_id, *a_id);
        self
          .vars
          .try_union(e.0, a.0)
          .map(|root| {
            let id = TypeVarId(root.id());
            InferredTy::new(self.ctx.inferred_kinds().alloc(InferredTyKind::Var(id)))
          })
          .map_err(|_| self.mismatch_error(expected, actual, types))
      }

      // Non-var vs var — bind the var.
      (InferredTyKind::Ty(_) | InferredTyKind::Compound(_), InferredTyKind::Var(var_id)) => {
        let var_id = *var_id;
        self
          .bind_var_to_inferred(var_id, expected, types)
          .map(|_| expected)
          .map_err(|_| self.mismatch_error(expected, actual, types))
      }
      (InferredTyKind::Var(var_id), InferredTyKind::Ty(_) | InferredTyKind::Compound(_)) => {
        let var_id = *var_id;
        self
          .bind_var_to_inferred(var_id, actual, types)
          .map(|_| actual)
          .map_err(|_| self.mismatch_error(expected, actual, types))
      }
    }
  }

  fn unify_kinds(
    &mut self,
    k_e: &TyKind<InferredTy<'ctx>>,
    k_a: &TyKind<InferredTy<'ctx>>,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
    types: &mut TypeRegistry<'ctx>,
  ) -> TypeCheckerResult<'ctx, InferredTy<'ctx>> {
    match (k_e, k_a) {
      (TyKind::Unit, TyKind::Unit) => Ok(expected),
      (TyKind::Primitive(p_e), TyKind::Primitive(p_a)) if p_e == p_a => Ok(expected),
      (TyKind::Function(f_e), TyKind::Function(f_a)) => {
        if f_e.parameters().len() != f_a.parameters().len() {
          return Err(TypeCheckerError::ArityMismatch {
            expected: f_e.parameters().len(),
            actual: f_a.parameters().len(),
          });
        }
        let pairs = f_e
          .parameters()
          .iter()
          .copied()
          .zip(f_a.parameters().iter().copied());
        for (pe, pa) in pairs {
          self.unify(pe, pa, types)?;
        }
        self.unify(f_e.return_type(), f_a.return_type(), types)?;
        Ok(expected)
      }
      (TyKind::Struct(s_e), TyKind::Struct(s_a)) => {
        if s_e.fields().len() != s_a.fields().len() {
          return Err(self.mismatch_error(expected, actual, types));
        }
        for (fe, fa) in s_e.fields().iter().zip(s_a.fields().iter()) {
          if fe.name() != fa.name() {
            return Err(self.mismatch_error(expected, actual, types));
          }
          self.unify(fe.ty(), fa.ty(), types)?;
        }
        Ok(expected)
      }
      _ => Err(self.mismatch_error(expected, actual, types)),
    }
  }

  fn bind_var_to_inferred(
    &mut self,
    var_id: TypeVarId,
    ty: InferredTy<'ctx>,
    types: &mut TypeRegistry<'ctx>,
  ) -> Result<(), TypeVarMergeError> {
    let concrete = self.resolve(ty, types);
    self.bind_var(var_id, concrete)
  }

  pub fn check_requirement(
    &self,
    ty: InferredTy<'ctx>,
    requirement: TypeClass,
    types: &mut TypeRegistry<'ctx>,
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
    match ty.kind() {
      InferredTyKind::Ty(ty) => requirement.accepts_ty(*ty),
      InferredTyKind::Compound(kind) => requirement.accepts_kind(kind),
      InferredTyKind::Var(id) => match self.type_var(*id) {
        TypeVar::Bound(ty) => requirement.accepts_ty(ty),
        TypeVar::Unbound(constraint) => requirement.accepts_constraint(constraint),
      },
    }
  }

  fn mismatch_error(
    &self,
    expected: InferredTy<'ctx>,
    actual: InferredTy<'ctx>,
    types: &mut TypeRegistry<'ctx>,
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
    self.accepts_kind(ty.deref())
  }

  fn accepts_kind<T>(&self, kind: &TyKind<T>) -> bool {
    use PrimitiveType::*;
    match self {
      Self::Numeric => matches!(kind, TyKind::Primitive(I32 | I64 | F32 | F64)),
      Self::Eq => matches!(kind, TyKind::Primitive(I32 | I64 | Bool)),
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
    use TypeVar::*;

    let (Unbound(this), Bound(other)) = (self, other) else {
      return false;
    };

    matches!(
      (this, other.deref()),
      (Floating, TyKind::Primitive(F32 | F64)) | (Integral, TyKind::Primitive(I32 | I64))
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

  use crate::{
    parser::token::ident::{Ident, matchers::ident},
    type_checker::{
      context::TypeCheckerCtx,
      error::matchers::{arity_mismatch_error, type_mismatch_error},
      types::{
        function::{
          FunctionType,
          matchers::{fn_param_types, fn_return_type},
        },
        inferred_ty::InferredTy,
        primitive::matchers::{bool_type, f32_type, f64_type, i32_type, i64_type},
        strukt::{
          StructField, StructType,
          matchers::{struct_field, struct_fields},
        },
        ty_kind::TyKind,
      },
    },
  };

  fn make_inferred_fn<'ctx>(
    inference: &InferenceTable<'ctx>,
    params: Vec<InferredTy<'ctx>>,
    return_type: InferredTy<'ctx>,
  ) -> InferredTy<'ctx> {
    InferredTy::new(
      inference
        .ctx
        .inferred_kinds()
        .alloc(InferredTyKind::Compound(TyKind::Function(
          FunctionType::new(params, return_type),
        ))),
    )
  }

  fn make_inferred_struct<'ctx>(
    inference: &InferenceTable<'ctx>,
    fields: Vec<StructField<InferredTy<'ctx>>>,
  ) -> InferredTy<'ctx> {
    InferredTy::new(
      inference
        .ctx
        .inferred_kinds()
        .alloc(InferredTyKind::Compound(TyKind::Struct(StructType::new(
          fields,
        )))),
    )
  }

  #[gtest]
  fn unresolved_numeric_vars_resolve_to_default_types() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let integral = inference.new_integral_var();
    let floating = inference.new_floating_var();

    expect_that!(&inference.resolve(integral, &mut types), i32_type());
    expect_that!(&inference.resolve(floating, &mut types), f32_type());
  }

  #[gtest]
  fn unify_works_across_multiple_variables() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let var0 = inference.new_integral_var();
    let var1 = inference.new_integral_var();
    let i64_ty = inference.lift_ty(types.primitive_type(PrimitiveType::I64));
    inference.unify(var0, var1, &mut types).unwrap();
    inference.unify(var1, i64_ty, &mut types).unwrap();

    expect_that!(&inference.resolve(var0, &mut types), i64_type());
  }

  #[gtest]
  fn unify_rejects_incompatible_bound_types() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let i32 = inference.lift_ty(types.primitive_type(PrimitiveType::I32));
    let bool = inference.lift_ty(types.primitive_type(PrimitiveType::Bool));

    expect_that!(
      inference.unify(i32, bool, &mut types),
      err(type_mismatch_error(i32_type(), bool_type()))
    );
  }

  #[gtest]
  fn unify_rejects_incompatible_variables() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let integral_var = inference.new_integral_var();
    let floating_var = inference.new_floating_var();

    expect_that!(
      inference.unify(integral_var, floating_var, &mut types),
      err(type_mismatch_error(i32_type(), f32_type()))
    );
  }

  #[gtest]
  fn unify_rejects_incompatible_numeric_constraint() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let integral = inference.new_integral_var();
    let f64 = inference.lift_ty(types.primitive_type(PrimitiveType::F64));

    expect_that!(
      inference.unify(integral, f64, &mut types),
      err(type_mismatch_error(i32_type(), f64_type()))
    );
  }

  #[gtest]
  fn lifted_ty_resolves_to_original_ty() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let i64_ty = types.primitive_type(PrimitiveType::I64);
    expect_that!(
      &inference.resolve(inference.lift_ty(i64_ty), &mut types),
      i64_type()
    );
  }

  #[gtest]
  fn inferred_function_type_resolves_to_concrete() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let inference = InferenceTable::new(&ctx);

    let param = inference.lift_ty(types.primitive_type(PrimitiveType::I32));
    let ret = inference.lift_ty(types.primitive_type(PrimitiveType::Bool));
    let fn_ty = make_inferred_fn(&inference, vec![param], ret);

    expect_that!(
      &inference.resolve(fn_ty, &mut types),
      all![
        fn_param_types(elements_are![i32_type()]),
        fn_return_type(bool_type())
      ]
    );
  }

  #[gtest]
  fn inferred_function_vars_use_defaults_on_resolve() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let param_var = inference.new_integral_var();
    let ret_var = inference.new_floating_var();
    let fn_ty = make_inferred_fn(&inference, vec![param_var], ret_var);

    expect_that!(
      &inference.resolve(fn_ty, &mut types),
      all![
        fn_param_types(elements_are![i32_type()]),
        fn_return_type(f32_type())
      ]
    );
  }

  #[gtest]
  fn inferred_function_vars_bind_through_unification() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let param_var = inference.new_integral_var();
    let ret_var = inference.new_floating_var();
    let inferred = make_inferred_fn(&inference, vec![param_var], ret_var);

    let i64 = inference.lift_ty(types.primitive_type(PrimitiveType::I64));
    let f64 = inference.lift_ty(types.primitive_type(PrimitiveType::F64));
    let concrete = make_inferred_fn(&inference, vec![i64], f64);

    inference.unify(inferred, concrete, &mut types).unwrap();

    expect_that!(
      &inference.resolve(inferred, &mut types),
      all![
        fn_param_types(elements_are![i64_type()]),
        fn_return_type(f64_type())
      ]
    );
  }

  #[gtest]
  fn function_types_reject_arity_mismatch() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let a = inference.new_integral_var();
    let b = inference.new_integral_var();
    let two_params = make_inferred_fn(&inference, vec![a, b], a);
    let one_param = make_inferred_fn(&inference, vec![a], b);

    expect_that!(
      inference.unify(two_params, one_param, &mut types),
      err(arity_mismatch_error(eq(&2), eq(&1)))
    );
  }

  #[gtest]
  fn concrete_fn_unifies_with_inferred_fn_and_binds_vars() {
    // Exercises unify_concrete_with_kind: Ty(fn_ty) vs Compound(Function([var], var)).
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let concrete_fn = types.function_type(
      [types.primitive_type(PrimitiveType::I32)],
      types.primitive_type(PrimitiveType::I64),
    );
    let param_var = inference.new_integral_var();
    let ret_var = inference.new_integral_var();
    let inferred = make_inferred_fn(&inference, vec![param_var], ret_var);

    inference
      .unify(inference.lift_ty(concrete_fn), inferred, &mut types)
      .unwrap();

    expect_that!(&inference.resolve(param_var, &mut types), i32_type());
    expect_that!(&inference.resolve(ret_var, &mut types), i64_type());
  }

  #[gtest]
  fn inferred_struct_type_resolves_to_concrete() {
    let ctx = TypeCheckerCtx::default();
    let mut types = TypeRegistry::new(&ctx);
    let mut inference = InferenceTable::new(&ctx);

    let field_var = inference.new_integral_var();
    let flag = inference.lift_ty(types.primitive_type(PrimitiveType::Bool));
    let struct_ty = make_inferred_struct(
      &inference,
      vec![
        StructField::new(Ident::new_isolated("count"), field_var),
        StructField::new(Ident::new_isolated("flag"), flag),
      ],
    );

    expect_that!(
      &inference.resolve(struct_ty, &mut types),
      struct_fields(elements_are![
        struct_field(ident("count"), i32_type()),
        struct_field(ident("flag"), bool_type()),
      ])
    );
  }
}
