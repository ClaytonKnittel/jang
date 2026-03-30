use crate::parser::{
  ast::{statement::Statement, type_expr::Type},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct FunctionDecl {
  name: Ident,
  parameters: Vec<FunctionParameter>,
  return_type: Type,
  body: Vec<Statement>,
}

impl FunctionDecl {
  pub fn new(
    name: Ident,
    parameters: Vec<FunctionParameter>,
    return_type: Type,
    body: Vec<Statement>,
  ) -> Self {
    Self {
      name,
      parameters,
      return_type,
      body,
    }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn parameters(&self) -> &[FunctionParameter] {
    &self.parameters
  }

  pub fn return_type(&self) -> &Type {
    &self.return_type
  }

  pub fn body(&self) -> &[Statement] {
    &self.body
  }
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
  name: Ident,
  ty: Type,
}

impl FunctionParameter {
  pub fn new(name: Ident, ty: Type) -> Self {
    Self { name, ty }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn ty(&self) -> &Type {
    &self.ty
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::{
    ast::{
      function_decl::{FunctionDecl, FunctionParameter},
      statement::Statement,
      type_expr::Type,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn fn_name_matches<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.name(), matcher)
  }

  pub fn fn_return_type_matches<'a>(
    matcher: impl Matcher<&'a Type>,
  ) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.return_type(), matcher)
  }

  pub fn fn_body_matches<'a>(
    matcher: impl Matcher<&'a [Statement]>,
  ) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.body(), matcher)
  }

  pub fn fn_parameters_match<'a>(
    matcher: impl Matcher<&'a [FunctionParameter]>,
  ) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.parameters(), matcher)
  }

  pub fn fn_parameter_name_matches<'a>(
    matcher: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.name(), matcher)
  }

  pub fn fn_parameter_type_matches<'a>(
    matcher: impl Matcher<&'a Type>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.ty(), matcher)
  }
}
