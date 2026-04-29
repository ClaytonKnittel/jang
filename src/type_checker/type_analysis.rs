use crate::type_checker::{
  typed_ast_id::{TypedAstId, TypedAstIdTable},
  types::registry::{TypeId, TypeRegistry},
};

pub struct JangTypeAnalysis {
  registry: TypeRegistry,
  ast_types: TypedAstIdTable<TypeId>,
}

impl JangTypeAnalysis {
  pub fn new(registry: TypeRegistry, ast_types: TypedAstIdTable<TypeId>) -> Self {
    Self {
      registry,
      ast_types,
    }
  }

  pub fn get(&self, id: impl Into<TypedAstId>) -> TypeId {
    *self
      .ast_types
      .get(id)
      .expect("Concrete type must be available after type checking")
  }

  pub fn registry(&self) -> &TypeRegistry {
    &self.registry
  }
}
