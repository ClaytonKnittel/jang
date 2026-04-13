use crate::parser::{
  ast::{
    expression::{Expression, ExpressionVariant},
    id::builder::IdBuilder,
    var_decl::{GlobalDecl, LocalDecl},
  },
  token::ident::Ident,
};

#[derive(Default)]
pub struct AstBuilderContext {
  id_builder: IdBuilder,
}

impl AstBuilderContext {
  pub fn new_expression(&mut self, variant: ExpressionVariant) -> Expression {
    Expression::new(self.id_builder.new_expr_id(), variant)
  }

  pub fn new_global_decl(&mut self, name: Ident) -> GlobalDecl {
    GlobalDecl::new(self.id_builder.new_global_decl_id(), name)
  }

  pub fn new_local_decl(&mut self, name: Ident) -> LocalDecl {
    LocalDecl::new(self.id_builder.new_local_decl_id(), name)
  }
}
