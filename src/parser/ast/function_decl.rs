use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::ast::{
  block::Block,
  type_expr::TypeExpression,
  var_decl::{GlobalDecl, LocalDecl},
};

#[derive(Clone, Debug)]
pub struct FunctionDecl {
  name: GlobalDecl,
  parameters: FunctionParameters,
  return_type: Option<TypeExpression>,
  body: Block,
}

impl FunctionDecl {
  pub fn new(
    name: GlobalDecl,
    parameters: FunctionParameters,
    return_type: Option<TypeExpression>,
    body: Block,
  ) -> Self {
    Self {
      name,
      parameters,
      return_type,
      body,
    }
  }

  pub fn name_decl(&self) -> &GlobalDecl {
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
  var: LocalDecl,
  ty: TypeExpression,
}

impl FunctionParameter {
  pub fn new(var: LocalDecl, ty: TypeExpression) -> Self {
    Self { var, ty }
  }

  pub fn var(&self) -> &LocalDecl {
    &self.var
  }

  pub fn ty(&self) -> &TypeExpression {
    &self.ty
  }
}

impl Display for FunctionParameter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.var, self.ty)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      block::Block,
      function_decl::{FunctionDecl, FunctionParameter},
      type_expr::TypeExpression,
      var_decl::matchers::{global_decl, local_decl},
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn fn_name<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a FunctionDecl> {
    property!(&FunctionDecl.name_decl(), global_decl(matcher))
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
    property!(&FunctionParameter.var(), local_decl(matcher))
  }

  pub fn fn_parameter_type<'a>(
    matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a FunctionParameter> {
    property!(&FunctionParameter.ty(), matcher)
  }
}
