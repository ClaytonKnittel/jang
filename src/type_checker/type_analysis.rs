use crate::type_checker::{
  typed_ast_id::{TypedAstId, TypedAstIdTable},
  types::concrete::ConcreteType,
};

pub struct JangTypeAnalysis {
  ast_types: TypedAstIdTable<ConcreteType>,
}

impl JangTypeAnalysis {
  pub fn new(ast_types: TypedAstIdTable<ConcreteType>) -> Self {
    Self { ast_types }
  }

  pub fn get(&self, id: impl Into<TypedAstId>) -> &ConcreteType {
    self
      .ast_types
      .get(id)
      .expect("Concrete type must be available after type checking")
  }
}
