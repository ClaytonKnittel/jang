use std::fmt::Display;

use cknittel_util::builder::Builder;
use itertools::Itertools;

use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Debug, Clone)]
pub struct StructFieldInitializer {
  name: Ident,
  value: Box<Expression>,
}

impl StructFieldInitializer {
  pub fn new(name: Ident, value: impl Into<Expression>) -> Self {
    Self {
      name,
      value: Box::new(value.into()),
    }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn expr(&self) -> &Expression {
    &self.value
  }
}

impl Display for StructFieldInitializer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.value)
  }
}

#[derive(Debug, Clone, Builder)]
pub struct StructExpression {
  #[vec]
  fields: Vec<StructFieldInitializer>,
}

impl StructExpression {
  pub fn new(fields: impl IntoIterator<Item = StructFieldInitializer>) -> Self {
    Self {
      fields: fields.into_iter().collect(),
    }
  }

  pub fn fields(&self) -> &[StructFieldInitializer] {
    &self.fields
  }
}

impl Display for StructExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.fields.iter().format(" "))
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use std::ops::Deref;

  use crate::parser::{
    ast::{
      expression::{Expression, ExpressionVariant, matchers::expr_variant},
      struct_expression::{StructExpression, StructFieldInitializer},
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn struct_initializer_field<'a>(
    name: impl Matcher<&'a Ident>,
    value: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a StructFieldInitializer> {
    pat!(StructFieldInitializer {
      name: name,
      value: result_of!(Box::deref, value),
    })
  }

  pub fn struct_exp<'a>(
    fields: impl Matcher<&'a [StructFieldInitializer]>,
  ) -> impl Matcher<&'a Expression> {
    expr_variant(pat!(ExpressionVariant::AnonymousStruct(result_of!(
      &StructExpression::fields,
      fields
    ))))
  }
}
