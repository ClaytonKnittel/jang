use crate::parser::{
  ast::id::{
    AstIdImpl,
    def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId},
    node_map::{NodeMap, NodeMapBuilder},
  },
  token::ident::Ident,
};

pub type IdTable = NodeMap<(), Ident, Ident>;

struct IdGenerator<ID, T> {
  cur_id: ID,
  table: Vec<T>,
}

impl<ID: AstIdImpl, T> IdGenerator<ID, T> {
  fn next_id(&mut self, value: T) -> ID {
    self.table.push(value);

    let id = self.cur_id;
    self.cur_id = id.next_id();
    id
  }
}

impl<ID: AstIdImpl, T> Default for IdGenerator<ID, T> {
  fn default() -> Self {
    Self {
      cur_id: ID::default(),
      table: Vec::new(),
    }
  }
}

#[derive(Default)]
pub struct IdBuilder {
  expressions: IdGenerator<AstExpressionId, ()>,
  globals: IdGenerator<AstGlobalDeclId, Ident>,
  locals: IdGenerator<AstLocalDeclId, Ident>,
}

impl IdBuilder {
  pub fn new_expr(&mut self) -> AstExpressionId {
    self.expressions.next_id(())
  }

  pub fn new_global(&mut self, ident: Ident) -> AstGlobalDeclId {
    self.globals.next_id(ident)
  }

  pub fn new_local(&mut self, ident: Ident) -> AstLocalDeclId {
    self.locals.next_id(ident)
  }

  pub fn build(self) -> IdTable {
    let mut builder = NodeMapBuilder::default();
    builder.set_expressions(self.expressions.table);
    builder.set_globals(self.globals.table);
    builder.set_locals(self.locals.table);
    builder.build().expect("All values should have been set")
  }
}

pub enum AstDeclId {
  Global(AstGlobalDeclId),
  Local(AstLocalDeclId),
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::parser::ast::id::builder::IdBuilder;

  #[gtest]
  fn test_ast_expr_id() {
    let mut ctx = IdBuilder::default();
    let expr_id_a = ctx.new_expr();
    let expr_id_b = ctx.new_expr();
    expect_eq!(expr_id_a.0, 0);
    expect_eq!(expr_id_b.0, 1);
  }
}
