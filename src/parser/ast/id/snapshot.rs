use crate::parser::ast::id::{
  def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId},
  range::AstIdRange,
};

#[derive(Clone, Debug)]
pub struct IdSnapshot {
  pub(super) expression: AstExpressionId,
  pub(super) global: AstGlobalDeclId,
  pub(super) local: AstLocalDeclId,
}

impl IdSnapshot {
  pub fn to(self, end: IdSnapshot) -> AstIdRange {
    AstIdRange { start: self, end }
  }
}
