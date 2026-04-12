use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::{
  ast::{
    block::Block,
    ids::{AstGlobalDeclId, AstLocalDeclId},
    type_expr::TypeExpression,
  },
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct FunctionDecl {
  decl_id: AstGlobalDeclId,
  name: Ident,
  parameters: FunctionParameters,
  return_type: Option<TypeExpression>,
  body: Block,
}

impl FunctionDecl {
  pub fn new(
    decl_id: AstGlobalDeclId,
    name: Ident,
    parameters: FunctionParameters,
    return_type: Option<TypeExpression>,
    body: Block,
  ) -> Self {
    Self {
      decl_id,
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
    self.parameters.parameters()
  }

  pub fn return_type(&self) -> Option<&TypeExpression> {
    self.return_type.as_ref()
  }

  pub fn body(&self) -> &Block {
    &self.body
  }

  pub fn decl_id(&self) -> AstGlobalDeclId {
    self.decl_id
  }
}

impl Display for FunctionDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.return_type {
      Some(ty) => write!(f, "fn {}({}) -> {} ", self.name, self.parameters, ty)?,
      None => write!(f, "fn {}({}) ", self.name, self.parameters)?,
    }
    writeln!(f, "{}", self.body)
  }
}

#[derive(Clone, Debug, Builder)]
pub struct FunctionParameters {
  #[vec]
  parameters: Vec<FunctionParameter>,
}

impl FunctionParameters {
  pub fn parameters(&self) -> &[FunctionParameter] {
    &self.parameters
  }
}

impl Display for FunctionParameters {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut first = true;
    for parameter in &self.parameters {
      if first {
        first = false;
      } else {
        write!(f, ", ")?;
      }

      write!(f, "{parameter}")?;
    }
    Ok(())
  }
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
  decl_id: AstLocalDeclId,
  name: Ident,
  ty: TypeExpression,
}

impl FunctionParameter {
  pub fn new(decl_id: AstLocalDeclId, name: Ident, ty: TypeExpression) -> Self {
    Self { decl_id, name, ty }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn ty(&self) -> &TypeExpression {
    &self.ty
  }

  pub fn decl_id(&self) -> AstLocalDeclId {
    self.decl_id
  }
}

impl Display for FunctionParameter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.name, self.ty)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      block::Block,
      function_decl::{FunctionDecl, FunctionParameter},
      type_expr::TypeExpression,
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

  pub fn fn_return_type<'a>(
    matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a FunctionDecl> {
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
    matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.ty(), matcher)
  }
}
