use std::{
  collections::HashSet,
  fmt::Display,
  hash::{Hash, Hasher},
  ops::Deref,
};

use enum_map::EnumMap;

use crate::type_checker::{
  context::TypeCheckerCtx,
  types::{
    function::FunctionType,
    primitive::PrimitiveType,
    strukt::{StructField, StructType},
    ty_kind::TyKind,
  },
};

/// A reference to a unique [`TyKind`] allocated through a
/// [`TypeRegistry`]. [`Ty`] is copyable and equality is cheap.
#[derive(Clone, Copy, Debug, Eq)]
pub struct Ty<'ctx>(&'ctx TyKind<Ty<'ctx>>);

impl<'ctx> Deref for Ty<'ctx> {
  type Target = TyKind<Ty<'ctx>>;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<'ctx> PartialEq for Ty<'ctx> {
  fn eq(&self, other: &Self) -> bool {
    // Instances are unique and only constructable via TypeRegistry,
    // so it's safe to just check for pointer equality.
    let ptr_eq = std::ptr::eq(self.0, other.0);

    // In debug mode, always verify that pointer equality
    // is identical to structural equality.
    debug_assert!({
      let structural_eq = self.0 == other.0;
      ptr_eq == structural_eq
    });

    ptr_eq
  }
}

impl<'ctx> Hash for Ty<'ctx> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.deref().hash(state)
  }
}

impl<'ctx> Display for Ty<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

/// A registry of unique types.
///
/// Responsible for creating new [`Ty`] objects for Jang's types.
pub struct TypeRegistry<'ctx> {
  type_set: TypeSet<'ctx>,
  unit_type: Ty<'ctx>,
  primitives: EnumMap<PrimitiveType, Ty<'ctx>>,
}

impl<'ctx> TypeRegistry<'ctx> {
  pub fn new(ctx: &'ctx TypeCheckerCtx<'ctx>) -> Self {
    let mut type_set = TypeSet::new(ctx);
    let unit_type = type_set.add(TyKind::Unit);
    let primitives = enum_map::enum_map! { p => type_set.add(TyKind::Primitive(p)) };
    Self {
      type_set,
      unit_type,
      primitives,
    }
  }

  pub fn unit_type(&self) -> Ty<'ctx> {
    self.unit_type
  }

  pub fn primitive_type(&self, primitive: PrimitiveType) -> Ty<'ctx> {
    self.primitives[primitive]
  }

  /// Adds a function type to the registry and returns its handle,
  /// deduplicating if the function already exists.
  pub fn function_type(
    &mut self,
    parameters: impl IntoIterator<Item = Ty<'ctx>>,
    return_type: Ty<'ctx>,
  ) -> Ty<'ctx> {
    self.type_set.add(TyKind::Function(FunctionType::new(
      parameters.into_iter().collect(),
      return_type,
    )))
  }

  /// Adds a struct type to the registry and returns its handle,
  /// deduplicating if the function already exists.
  pub fn struct_type(
    &mut self,
    fields: impl IntoIterator<Item = StructField<Ty<'ctx>>>,
  ) -> Ty<'ctx> {
    self.type_set.add(TyKind::Struct(StructType::new(fields)))
  }
}

/// A set of unique types.
/// Facilitates allocation and deduplication of [`Ty`] values.
struct TypeSet<'ctx> {
  ctx: &'ctx TypeCheckerCtx<'ctx>,

  /// For deduplication.
  type_set: HashSet<&'ctx TyKind<Ty<'ctx>>>,
}

impl<'ctx> TypeSet<'ctx> {
  fn new(ctx: &'ctx TypeCheckerCtx<'ctx>) -> Self {
    Self {
      ctx,
      type_set: HashSet::new(),
    }
  }

  /// Adds a new type to the set, deduplicating against existing entries.
  fn add(&mut self, kind: TyKind<Ty<'ctx>>) -> Ty<'ctx> {
    if let Some(ty) = self.type_set.get(&kind) {
      return Ty(ty);
    }
    let allocated: &'ctx TyKind<Ty<'ctx>> = self.ctx.types().alloc(kind);
    self.type_set.insert(allocated);
    Ty(allocated)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  /// Lifts a matcher on `&TyKind<Ty>` to a matcher on `&Ty`.
  pub fn ty<'a>(matcher: impl Matcher<&'a TyKind<Ty<'a>>>) -> impl Matcher<&'a Ty<'a>> {
    result_of!(|ty: &'a Ty<'a>| ty.deref(), matcher)
  }
}
