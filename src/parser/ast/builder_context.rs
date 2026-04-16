use crate::{
  error::JangResult,
  parser::{
    ast::{
      expression::{Expression, ExpressionVariant},
      id::{builder::IdBuilder, id_counts::AstIdCounts},
      var::{
        var_decl::{GlobalDecl, LocalDecl},
        var_decl_map::VarDeclMap,
        var_ref::VarRef,
      },
    },
    token::ident::Ident,
  },
};

#[derive(Default)]
pub struct AstBuilderContext {
  id_builder: IdBuilder,
  var_decl_map: VarDeclMap,
}

impl AstBuilderContext {
  pub fn new_expression(&mut self, variant: ExpressionVariant) -> Expression {
    Expression::new(self.id_builder.new_expr_id(), variant)
  }

  pub fn new_global_decl(&mut self, ident: Ident) -> JangResult<GlobalDecl> {
    self
      .var_decl_map
      .new_global_decl(ident, &mut self.id_builder)
  }

  pub fn new_local_decl(&mut self, ident: Ident) -> LocalDecl {
    self
      .var_decl_map
      .new_local_decl(ident, &mut self.id_builder)
  }

  pub fn new_var_ref(&mut self, ident: Ident) -> VarRef {
    self.var_decl_map.var_ref(ident, &mut self.id_builder)
  }

  pub fn enter_block_scope(&mut self) {
    self.var_decl_map.enter_block_scope();
  }

  pub fn exit_block_scope(&mut self) {
    self.var_decl_map.exit_block_scope();
  }

  pub fn id_counts(&self) -> AstIdCounts {
    self.id_builder.id_counts()
  }
}
