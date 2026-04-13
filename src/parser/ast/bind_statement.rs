use std::fmt::Display;

use crate::parser::ast::{expression::Expression, var::var_decl::LocalDecl};

#[derive(Copy, Clone, Debug)]
pub enum Mutability {
  Mutable,
  Immutable,
}

#[derive(Clone, Debug)]
pub struct BindStatement {
  mutability: Mutability,
  var: LocalDecl,
  expr: Expression,
}

impl BindStatement {
  pub fn new_let(var: LocalDecl, expr: Expression) -> Self {
    Self {
      mutability: Mutability::Immutable,
      var,
      expr,
    }
  }

  pub fn new_mut(var: LocalDecl, expr: Expression) -> Self {
    Self {
      mutability: Mutability::Mutable,
      var,
      expr,
    }
  }

  pub fn mutability(&self) -> &Mutability {
    &self.mutability
  }

  pub fn var(&self) -> &LocalDecl {
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
      var::var_decl::matchers::local_decl,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn let_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      var: local_decl(var_matcher),
      expr: expr_matcher,
      mutability: pat!(Mutability::Immutable)
    })))
  }

  pub fn mut_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      var: local_decl(var_matcher),
      expr: expr_matcher,
      mutability: pat!(Mutability::Mutable)
    })))
  }
}
