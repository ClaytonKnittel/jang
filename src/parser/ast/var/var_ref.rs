use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::{
  ast::{
    id::def::AstDeclId,
    var::var_decl::{GlobalDecl, LocalDecl},
  },
  token::ident::Ident,
};

#[derive(Clone, Debug, FromVariants)]
pub enum VarRef {
  Global(GlobalDecl),
  Local(LocalDecl),
}

impl VarRef {
  pub(super) fn from_id(id: AstDeclId, name: Ident) -> Self {
    match id {
      AstDeclId::Global(id) => Self::Global(GlobalDecl::new(id, name)),
      AstDeclId::Local(id) => Self::Local(LocalDecl::new(id, name)),
    }
  }

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
  use crate::parser::{
    ast::{
      expression::{Expression, ExpressionVariant, matchers::expr_variant},
      var::var_ref::VarRef,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn var_ref<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a VarRef> {
    property!(&VarRef.name(), matcher)
  }

  pub fn var_ref_expr<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a Expression> {
    expr_variant(pat!(ExpressionVariant::VarRef(var_ref(matcher))))
  }
}
