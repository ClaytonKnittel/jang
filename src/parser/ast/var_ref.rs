use std::fmt::Display;

use crate::parser::{
  ast::var_decl::{GlobalDecl, LocalDecl},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub enum VarRef {
  Global(GlobalDecl),
  Local(LocalDecl),
}

impl VarRef {
  pub fn name(&self) -> &Ident {
    match self {
      Self::Global(global_decl) => global_decl.name(),
      Self::Local(local_decl) => local_decl.name(),
    }
  }
}

impl Display for VarRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Global(global_decl) => write!(f, "{global_decl}"),
      Self::Local(local_decl) => write!(f, "{local_decl}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{ast::var_ref::VarRef, token::ident::Ident};
  use googletest::prelude::*;

  pub fn var_ref<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a VarRef> {
    property!(&VarRef.name(), matcher)
  }
}
