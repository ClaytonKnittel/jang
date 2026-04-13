use std::fmt::Display;

use crate::parser::{
  ast::id::{
    AstId,
    def::{AstGlobalDeclId, AstLocalDeclId},
  },
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct VarDecl<ID> {
  id: ID,
  name: Ident,
}

impl<ID: AstId> VarDecl<ID> {
  pub(super) fn new(id: ID, name: Ident) -> Self {
    Self { id, name }
  }

  pub fn id(&self) -> ID {
    self.id
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }
}

impl<ID> Display for VarDecl<ID> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

pub type GlobalDecl = VarDecl<AstGlobalDeclId>;
pub type LocalDecl = VarDecl<AstLocalDeclId>;

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::var_decl::{GlobalDecl, LocalDecl},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn global_decl<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a GlobalDecl> {
    property!(&GlobalDecl.name(), matcher)
  }

  pub fn local_decl<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a LocalDecl> {
    property!(&LocalDecl.name(), matcher)
  }
}
