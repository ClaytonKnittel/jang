use crate::type_checker::{
  typed_ast_id::{TypedAstId, TypedAstIdTable},
  types::registry::Ty,
};

pub struct JangTypeAnalysis<'a> {
  ast_types: TypedAstIdTable<Ty<'a>>,
}

impl<'a> JangTypeAnalysis<'a> {
  pub fn new(ast_types: TypedAstIdTable<Ty<'a>>) -> Self {
    Self { ast_types }
  }

  pub fn get(&self, id: impl Into<TypedAstId>) -> Ty<'a> {
    *self
      .ast_types
      .get(id)
      .expect("Concrete type must be available after type checking")
  }
}
