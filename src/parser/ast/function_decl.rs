use crate::parser::{
  ast::{block::Block, type_expr::Type},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct FunctionDecl {
  name: Ident,
  parameters: Vec<FunctionParameter>,
  return_type: Option<Type>,
  body: Block,
}

impl FunctionDecl {
  pub fn new(
    name: Ident,
    parameters: Vec<FunctionParameter>,
    return_type: Option<Type>,
    body: Block,
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

  pub fn return_type(&self) -> Option<&Type> {
    self.return_type.as_ref()
  }

  pub fn body(&self) -> &Block {
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
      block::Block,
      function_decl::{FunctionDecl, FunctionParameter},
      type_expr::Type,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn fn_name<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.name(), matcher)
  }

  pub fn fn_return_type_none<'a>() -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.return_type(), none())
  }

  pub fn fn_return_type<'a>(matcher: impl Matcher<&'a Type>) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.return_type(), some(matcher))
  }

  pub fn fn_body<'a>(matcher: impl Matcher<&'a Block>) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.body(), matcher)
  }

  pub fn fn_parameters<'a>(
    matcher: impl Matcher<&'a [FunctionParameter]>,
  ) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.parameters(), matcher)
  }

  pub fn fn_parameter_name<'a>(
    matcher: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.name(), matcher)
  }

  pub fn fn_parameter_type<'a>(
    matcher: impl Matcher<&'a Type>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.ty(), matcher)
  }
}
