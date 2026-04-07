use std::fmt::Display;

use cknittel_util::{builder::Builder, from_variants::FromVariants};

use crate::parser::{
  ast::{structured_type_decl::StructuredTypeDecl, type_decl::TypeDecl},
  token::ident::Ident,
};

#[derive(Clone, Debug, FromVariants)]
pub enum EnumVariantType {
  TypeRef(Ident),
  Structured(StructuredTypeDecl),
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
  name: Ident,
  ty: TypeDecl,
}

#[derive(Clone, Debug, Builder)]
pub struct EnumTypeDecl {
  #[vec]
  variants: Vec<EnumVariant>,
}

impl EnumTypeDecl {
  pub fn variants(&self) -> &[EnumVariant] {
    &self.variants
  }
}

impl Display for EnumTypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f)?;
    for variant in &self.variants {
      writeln!(f, "| {variant}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{structured_type_decl::StructuredTypeField, type_expr::TypeExpression},
    token::ident::Ident,
  };
  use googletest::prelude::*;
}
