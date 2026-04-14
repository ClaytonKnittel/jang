use crate::parser::{ast::id::node_map::NodeMap, token::ident::Ident};

pub type IdTable = NodeMap<(), Ident, Ident>;

pub trait AstId: Copy {}

pub(super) trait AstIdImpl: AstId {
  fn default() -> Self;

  fn next_id(&self) -> Self;
}
