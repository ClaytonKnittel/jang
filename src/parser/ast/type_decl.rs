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
