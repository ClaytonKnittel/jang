use std::fmt::Display;

use cknittel_util::{builder::Builder, from_variants::FromVariants};

use crate::parser::{ast::type_expr::TypeExpression, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct StructuredTypeField {
  name: Ident,
  ty: TypeExpression,
}

impl StructuredTypeField {
  pub fn new(name: Ident, ty: TypeExpression) -> Self {
    Self { name, ty }
  }
}

impl Display for StructuredTypeField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.ty)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct StructuredTypeDecl {
  #[vec]
  fields: Vec<StructuredTypeField>,
}

impl StructuredTypeDecl {
  pub fn fields(&self) -> &[StructuredTypeField] {
    &self.fields
  }
}

impl Display for StructuredTypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{{")?;
    for field in &self.fields {
      writeln!(f, "{field},")?;
    }
    write!(f, "}}")
  }
}

#[derive(Clone, Debug, FromVariants)]
pub enum TypeDeclVariant {
  Structured(StructuredTypeDecl),
}

impl Display for TypeDeclVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Structured(structured) => write!(f, "{structured}"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypeDecl {
  name: Ident,
  decl: TypeDeclVariant,
}

impl TypeDecl {
  pub fn new(name: Ident, decl: impl Into<TypeDeclVariant>) -> Self {
    let decl = decl.into();
    Self { name, decl }
  }
}

impl Display for TypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} = {}", self.name, self.decl)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      type_decl::{StructuredTypeDecl, StructuredTypeField, TypeDecl, TypeDeclVariant},
      type_expr::TypeExpression,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn type_field<'a>(
    name: impl Matcher<&'a Ident>,
    field_type: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a StructuredTypeField> {
    pat!(StructuredTypeField {
      name: name,
      ty: field_type,
    })
  }

  pub fn structured_type<'a>(
    name: impl Matcher<&'a Ident>,
    field_matchers: impl Matcher<&'a [StructuredTypeField]>,
  ) -> impl Matcher<&'a TypeDecl> {
    pat!(TypeDecl {
      name: name,
      decl: pat!(TypeDeclVariant::Structured(property!(
        &StructuredTypeDecl.fields(),
        field_matchers
      ))),
    })
  }
}
