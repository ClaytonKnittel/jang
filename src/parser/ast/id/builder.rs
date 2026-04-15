use crate::parser::ast::id::{
  AstIdImpl,
  def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId},
};

struct IdGenerator<ID> {
  cur_id: ID,
}

impl<ID: AstIdImpl> IdGenerator<ID> {
  fn next_id(&mut self) -> ID {
    let id = self.cur_id;
    self.cur_id = id.next_id();
    id
  }
}

impl<ID: AstIdImpl> Default for IdGenerator<ID> {
  fn default() -> Self {
    Self {
      cur_id: ID::default(),
    }
  }
}

#[derive(Default)]
pub struct IdBuilder {
  expressions: IdGenerator<AstExpressionId>,
  globals: IdGenerator<AstGlobalDeclId>,
  locals: IdGenerator<AstLocalDeclId>,
}

impl IdBuilder {
  pub fn new_expr_id(&mut self) -> AstExpressionId {
    self.expressions.next_id()
  }

  pub fn new_global_id(&mut self) -> AstGlobalDeclId {
    self.globals.next_id()
  }

  pub fn new_local_id(&mut self) -> AstLocalDeclId {
    self.locals.next_id()
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::parser::ast::id::builder::IdBuilder;

  #[gtest]
  fn test_ast_expr_id() {
    let mut ctx = IdBuilder::default();
    let expr_id_a = ctx.new_expr_id();
    let expr_id_b = ctx.new_expr_id();
    expect_eq!(expr_id_a.0, 0);
    expect_eq!(expr_id_b.0, 1);
  }
}
