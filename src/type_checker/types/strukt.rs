use std::fmt::Display;

use itertools::Itertools;

use crate::{parser::token::ident::Ident, type_checker::types::registry::Ty};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructField<'ctx> {
  name: Ident,
  ty: Ty<'ctx>,
}

impl<'ctx> StructField<'ctx> {
  pub fn new(name: impl Into<Ident>, ty: Ty<'ctx>) -> Self {
    StructField {
      name: name.into(),
      ty,
    }
  }
}

impl<'ctx> Display for StructField<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.ty)
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructType<'ctx> {
  fields: Vec<StructField<'ctx>>,
}

impl<'ctx> StructType<'ctx> {
  pub(super) fn new(fields: impl IntoIterator<Item = StructField<'ctx>>) -> Self {
    Self {
      fields: fields
        .into_iter()
        .sorted_by(|a, b| a.name.name().cmp(b.name.name()))
        .collect(),
    }
  }

  pub fn fields(&self) -> &[StructField<'ctx>] {
    &self.fields
  }

  pub fn field_ty(&self, name: &Ident) -> Option<Ty<'ctx>> {
    self.fields.iter().find(|f| &f.name == name).map(|f| f.ty)
  }
}

impl<'ctx> Display for StructType<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{ {} }}", self.fields.iter().format(", "))
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::type_checker::types::ty_kind::matchers::struct_type;

  use super::*;
  use googletest::prelude::*;

  pub fn struct_fields<'a>(
    fields: impl Matcher<&'a [StructField<'a>]>,
  ) -> impl Matcher<&'a Ty<'a>> {
    struct_type(result_of!(&StructType::fields, fields))
  }

  pub fn struct_field<'a>(
    name: impl Matcher<&'a Ident>,
    ty: impl Matcher<&'a Ty<'a>>,
  ) -> impl Matcher<&'a StructField<'a>> {
    pat!(StructField { name: name, ty: ty })
  }
}
