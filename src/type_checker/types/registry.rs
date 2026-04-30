use std::{
  collections::HashMap,
  fmt::Display,
  hash::{Hash, Hasher},
  ops::Deref,
};

use enum_map::EnumMap;

use crate::type_checker::{
  context::TypeCheckerCtx,
  types::{concrete::ConcreteType, function::FunctionType, primitive::PrimitiveType},
};

/// A reference to a unique [`ConcreteType`] allocated through a
/// [`TypeRegistry`]. [`Ty`] is copyable and equality is cheap.
#[derive(Clone, Copy, Debug, Eq)]
pub struct Ty<'a>(&'a ConcreteType<'a>);

impl<'a> Deref for Ty<'a> {
  type Target = ConcreteType<'a>;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<'a> PartialEq for Ty<'a> {
  fn eq(&self, other: &Self) -> bool {
    // Instances are unique and only constructable via TypeRegistry,
    // so it's safe to just check for pointer equality.
    std::ptr::eq(self.0, other.0)
  }
}

impl<'a> Hash for Ty<'a> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.deref().hash(state)
  }
}

impl<'a> Display for Ty<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.deref().fmt(f)
  }
}

/// A registry of unique types.
///
/// Responsible for creating new [`Ty`] objects for Jang's types.
pub struct TypeRegistry<'a> {
  type_set: TypeSet<'a>,
  unit_type: Ty<'a>,
  primitives: EnumMap<PrimitiveType, Ty<'a>>,
}

impl<'a> TypeRegistry<'a> {
  pub fn new(ctx: &'a TypeCheckerCtx<'a>) -> Self {
    let mut type_set = TypeSet::new(ctx);
    let unit_type = type_set.add(ConcreteType::Unit);
    let primitives = enum_map::enum_map! { p => type_set.add(ConcreteType::Primitive(p)) };
    Self {
      type_set,
      unit_type,
      primitives,
    }
  }

  pub fn unit_type(&self) -> Ty<'a> {
    self.unit_type
  }

  pub fn primitive_type(&self, primitive: PrimitiveType) -> Ty<'a> {
    self.primitives[primitive]
  }

  /// Adds a function type to the registry and returns its handle,
  /// deduplicating if the function already exists.
  pub fn function_type(&mut self, parameters: Vec<Ty<'a>>, return_type: Ty<'a>) -> Ty<'a> {
    self.type_set.add(ConcreteType::Function(FunctionType::new(
      parameters,
      return_type,
    )))
  }
}

/// A set of unique types.
/// Facilitates allocation and deduplication of [`Ty`] values.
struct TypeSet<'a> {
  ctx: &'a TypeCheckerCtx<'a>,

  /// For deduplication.
  type_refs: HashMap<&'a ConcreteType<'a>, Ty<'a>>,
}

impl<'a> TypeSet<'a> {
  fn new(ctx: &'a TypeCheckerCtx<'a>) -> Self {
    Self {
      ctx,
      type_refs: HashMap::new(),
    }
  }

  /// Adds a new type to the set, deduplicating against existing entries.
  fn add(&mut self, concrete: ConcreteType<'a>) -> Ty<'a> {
    if let Some(&ty) = self.type_refs.get(&concrete) {
      return ty;
    }
    let allocated: &'a ConcreteType<'a> = self.ctx.types().alloc(concrete);
    let ty = Ty(allocated);
    self.type_refs.insert(allocated, ty);
    ty
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  /// Lifts a matcher on `&ConcreteType` to a matcher on `&Ty`.
  pub fn ty<'a>(matcher: impl Matcher<&'a ConcreteType<'a>>) -> impl Matcher<&'a Ty<'a>> {
    result_of!(|ty: &'a Ty<'a>| ty.deref(), matcher)
  }
}
