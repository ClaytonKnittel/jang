use crate::parser::ast::id::{
  adt::map::IdMap,
  def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId},
  snapshot::IdSnapshot,
};

#[derive(Clone, Debug)]
pub struct AstIdRange {
  pub(super) start: IdSnapshot,
  pub(super) end: IdSnapshot,
}

impl AstIdRange {
  pub fn new_expression_id_map<T>(&self) -> IdMap<AstExpressionId, T> {
    IdMap::from_range(self.start.expression, self.end.expression)
  }

  pub fn new_local_id_map<T>(&self) -> IdMap<AstLocalDeclId, T> {
    IdMap::from_range(self.start.local, self.end.local)
  }

  pub fn new_global_id_map<T>(&self) -> IdMap<AstGlobalDeclId, T> {
    IdMap::from_range(self.start.global, self.end.global)
  }
}
