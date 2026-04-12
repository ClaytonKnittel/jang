use std::fmt::Display;

use crate::parser::{
  ast::{expression::Expression, ids::AstLocalDeclId},
  token::ident::Ident,
};

#[derive(Copy, Clone, Debug)]
pub enum Mutability {
  Mutable,
  Immutable,
}

#[derive(Clone, Debug)]
pub struct BindStatement {
  decl_id: AstLocalDeclId,
  mutability: Mutability,
  var: Ident,
  expr: Expression,
}

impl BindStatement {
  pub fn new_let(decl_id: AstLocalDeclId, var: Ident, expr: Expression) -> Self {
    Self {
      decl_id,
      mutability: Mutability::Immutable,
      var,
      expr,
    }
  }

  pub fn new_mut(decl_id: AstLocalDeclId, var: Ident, expr: Expression) -> Self {
    Self {
      decl_id,
      mutability: Mutability::Mutable,
      var,
      expr,
    }
  }

  pub fn decl_id(&self) -> AstLocalDeclId {
    self.decl_id
  }

  pub fn mutability(&self) -> &Mutability {
    &self.mutability
  }

  pub fn var(&self) -> &Ident {
    &self.var
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

impl Display for BindStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.mutability {
      Mutability::Mutable => write!(f, "mut {} = {}", self.var, self.expr),
      Mutability::Immutable => write!(f, "let {} = {}", self.var, self.expr),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      bind_statement::{BindStatement, Mutability},
      expression::Expression,
      statement::Statement,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn let_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      decl_id: anything(),
      var: var_matcher,
      expr: expr_matcher,
      mutability: pat!(Mutability::Immutable)
    })))
  }

  pub fn mut_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      decl_id: anything(),
      var: var_matcher,
      expr: expr_matcher,
      mutability: pat!(Mutability::Mutable)
    })))
  }
}
