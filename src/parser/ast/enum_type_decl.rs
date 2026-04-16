use std::fmt::Display;

use cknittel_util::{builder::Builder, from_variants::FromVariants};

use crate::parser::{ast::type_expr::TypeExpression, token::ident::Ident};

#[derive(Clone, Debug, FromVariants)]
pub enum EnumVariantType {
  TypeExpression(TypeExpression),
  Empty,
}

impl Display for EnumVariantType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TypeExpression(type_expr) => write!(f, "{type_expr}"),
      Self::Empty => Ok(()),
    }
  }
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
  name: Ident,
  ty: EnumVariantType,
}

impl EnumVariant {
  pub fn new(name: Ident, ty: EnumVariantType) -> Self {
    Self { name, ty }
  }
}

impl Display for EnumVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.name, self.ty)
  }
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
  use super::*;
  use crate::parser::{ast::type_expr::TypeExpression, token::ident::Ident};
  use googletest::prelude::*;

  pub fn enum_expr_type<'a>(
    type_expr_matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a EnumVariantType> {
    pat!(EnumVariantType::TypeExpression(type_expr_matcher))
  }

  pub fn enum_variant<'a>(name: impl Matcher<&'a Ident>) -> impl Matcher<&'a EnumVariant> {
    pat!(EnumVariant {
      name: name,
      ty: pat!(EnumVariantType::Empty)
    })
  }

  pub fn enum_variant_with<'a>(
    name: impl Matcher<&'a Ident>,
    type_matcher: impl Matcher<&'a EnumVariantType>,
  ) -> impl Matcher<&'a EnumVariant> {
    pat!(EnumVariant {
      name: name,
      ty: type_matcher
    })
  }
}
