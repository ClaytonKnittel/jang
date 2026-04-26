use std::{collections::HashMap, fmt, ops::Index};

use crate::type_checker::types::{concrete::ConcreteType, primitive::PrimitiveType};

/// Type ID.
/// Two types are equal within a registry iff they have the same type ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId(usize);

#[derive(Default)]
struct TypeSet {
  types: Vec<ConcreteType>,

  // For deduplication.
  type_ids: HashMap<ConcreteType, TypeId>,
}

/// A set of unique types.
impl TypeSet {
  /// Adds a new type to the registry.
  pub fn add(&mut self, ty: ConcreteType) -> TypeId {
    let id = *self
      .type_ids
      .entry(ty.clone())
      .or_insert(TypeId(self.types.len()));
    self.types.push(ty);
    id
  }
}

impl Index<TypeId> for TypeSet {
  type Output = ConcreteType;

  fn index(&self, index: TypeId) -> &Self::Output {
    &self.types[index.0]
  }
}

/// A registry of unique types.
/// Two types are considered the same iff they have the same Type ID.
pub struct TypeRegistry {
  types: TypeSet,

  // Primitives
  unit_type_id: TypeId,
  i32_type_id: TypeId,
  i64_type_id: TypeId,
  f32_type_id: TypeId,
  f64_type_id: TypeId,
  bool_type_id: TypeId,
}

impl Default for TypeRegistry {
  fn default() -> Self {
    let mut types = TypeSet::default();
    let unit_type_id = types.add(ConcreteType::Unit);
    let i32_type_id = types.add(ConcreteType::Primitive(PrimitiveType::I32));
    let i64_type_id = types.add(ConcreteType::Primitive(PrimitiveType::I64));
    let f32_type_id = types.add(ConcreteType::Primitive(PrimitiveType::F32));
    let f64_type_id = types.add(ConcreteType::Primitive(PrimitiveType::F64));
    let bool_type_id = types.add(ConcreteType::Primitive(PrimitiveType::Bool));
    Self {
      types,
      unit_type_id,
      i32_type_id,
      i64_type_id,
      f32_type_id,
      f64_type_id,
      bool_type_id,
    }
  }
}

impl TypeRegistry {
  pub fn unit_type(&self) -> TypeId {
    self.unit_type_id
  }

  pub fn bool_type(&self) -> TypeId {
    self.bool_type_id
  }

  pub fn i32_type(&self) -> TypeId {
    self.i32_type_id
  }

  pub fn i64_type(&self) -> TypeId {
    self.i64_type_id
  }

  pub fn f32_type(&self) -> TypeId {
    self.f32_type_id
  }

  pub fn f64_type(&self) -> TypeId {
    self.f64_type_id
  }

  /// Adds a new type to the registry.
  pub fn add(&mut self, ty: ConcreteType) -> TypeId {
    self.types.add(ty)
  }

  pub fn display(&self, type_id: TypeId) -> TypeDisplay<'_> {
    TypeDisplay {
      registry: self,
      type_id,
    }
  }
}

impl Index<TypeId> for TypeRegistry {
  type Output = ConcreteType;

  fn index(&self, index: TypeId) -> &Self::Output {
    &self.types[index]
  }
}

pub trait DisplayType {
  fn fmt_type(&self, f: &mut fmt::Formatter<'_>, registry: &TypeRegistry) -> fmt::Result;
}

impl DisplayType for TypeId {
  fn fmt_type(&self, f: &mut fmt::Formatter<'_>, registry: &TypeRegistry) -> fmt::Result {
    registry[*self].fmt_type(f, registry)
  }
}

pub struct TypeDisplay<'a> {
  registry: &'a TypeRegistry,
  type_id: TypeId,
}

impl<'a> fmt::Display for TypeDisplay<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.type_id.fmt_type(f, self.registry)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::type_checker::types::concrete::ConcreteType;
  use googletest::prelude::*;

  impl TypeRegistry {
    pub fn type_id<'a>(
      &'a self,
      matcher: impl Matcher<&'a ConcreteType>,
    ) -> impl Matcher<&'a TypeId> {
      result_of!(|id: &'a TypeId| &self[*id], matcher)
    }
  }
}
