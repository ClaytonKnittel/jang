use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::{
  ast::{enum_type_decl::EnumTypeDecl, type_expr::TypeExpression},
  token::ident::Ident,
};

#[derive(Clone, Debug, FromVariants)]
pub enum TypeDeclVariant {
  Enum(EnumTypeDecl),
  Alias(TypeExpression),
}

impl Display for TypeDeclVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Enum(enum_decl) => write!(f, "{enum_decl}"),
      Self::Alias(type_expr) => write!(f, "{type_expr}"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypeDecl {
  name: Ident,
  decl: TypeDeclVariant,
}

impl TypeDecl {
  pub fn new(name: Ident, decl: TypeDeclVariant) -> Self {
    Self { name, decl }
  }
}

impl Display for TypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "type {} = {}", self.name, self.decl)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::parser::{
    ast::{
      enum_type_decl::{EnumTypeDecl, EnumVariant},
      type_expr::TypeExpression,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn enum_type<'a>(
    name: impl Matcher<&'a Ident>,
    variant_matchers: impl Matcher<&'a [EnumVariant]>,
  ) -> impl Matcher<&'a TypeDecl> {
    pat!(TypeDecl {
      name: name,
      decl: pat!(TypeDeclVariant::Enum(property!(
        &EnumTypeDecl.variants(),
        variant_matchers
      ))),
    })
  }

  pub fn type_alias<'a>(
    name: impl Matcher<&'a Ident>,
    type_expr_matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a TypeDecl> {
    pat!(TypeDecl {
      name: name,
      decl: pat!(TypeDeclVariant::Alias(type_expr_matcher)),
    })
  }
}
