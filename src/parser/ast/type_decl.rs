use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::ast::type_expr::TypeExpression;

#[derive(Clone, Debug)]
pub struct StructuredTypeField {
  name: String,
  ty: TypeExpression,
}

impl Display for StructuredTypeField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.ty)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct StructuredTypeDecl {
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

#[derive(Clone, Debug)]
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
  name: String,
  decl: TypeDeclVariant,
}

impl Display for TypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} = {}", self.name, self.decl)
  }
}
