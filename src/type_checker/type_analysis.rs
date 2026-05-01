use crate::type_checker::{
  typed_ast_id::{TypedAstId, TypedAstIdTable},
  types::registry::Ty,
};

pub struct JangTypeAnalysis<'ctx> {
  ast_types: TypedAstIdTable<Ty<'ctx>>,
}

impl<'ctx> JangTypeAnalysis<'ctx> {
  pub fn new(ast_types: TypedAstIdTable<Ty<'ctx>>) -> Self {
    Self { ast_types }
  }

  pub fn get(&self, id: impl Into<TypedAstId>) -> Ty<'ctx> {
    *self
      .ast_types
      .get(id)
      .expect("Concrete type must be available after type checking")
  }
}
