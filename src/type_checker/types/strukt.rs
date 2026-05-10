use std::fmt::Display;

use itertools::Itertools;

use crate::parser::token::ident::Ident;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructField<Ty> {
  name: Ident,
  ty: Ty,
}

impl<Ty> StructField<Ty> {
  pub fn new(name: impl Into<Ident>, ty: Ty) -> Self {
    StructField {
      name: name.into(),
      ty,
    }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }
}

impl<Ty: Copy> StructField<Ty> {
  pub fn ty(&self) -> Ty {
    self.ty
  }
}

impl<Ty: Display> Display for StructField<Ty> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.ty)
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructType<Ty> {
  fields: Vec<StructField<Ty>>,
}

impl<Ty> StructType<Ty> {
  pub(crate) fn new(fields: impl IntoIterator<Item = StructField<Ty>>) -> Self {
    Self {
      fields: fields
        .into_iter()
        .sorted_by(|a, b| a.name.name().cmp(b.name.name()))
        .collect(),
    }
  }

  pub fn fields(&self) -> &[StructField<Ty>] {
    &self.fields
  }
}

impl<Ty: Copy + PartialEq> StructType<Ty> {
  pub fn field_ty(&self, name: &Ident) -> Option<Ty> {
    self.fields.iter().find(|f| &f.name == name).map(|f| f.ty)
  }
}

impl<Ty: Display> Display for StructType<Ty> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{ {} }}", self.fields.iter().format(", "))
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::type_checker::types::{registry::Ty, ty_kind::matchers::struct_type};

  use super::*;
  use googletest::prelude::*;

  pub fn struct_fields<'a>(
    fields: impl Matcher<&'a [StructField<Ty<'a>>]>,
  ) -> impl Matcher<&'a Ty<'a>> {
    struct_type(result_of!(&StructType::fields, fields))
  }

  pub fn struct_field<'a>(
    name: impl Matcher<&'a Ident>,
    ty: impl Matcher<&'a Ty<'a>>,
  ) -> impl Matcher<&'a StructField<Ty<'a>>> {
    pat!(StructField { name: name, ty: ty })
  }
}
