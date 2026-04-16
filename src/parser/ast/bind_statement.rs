use std::fmt::Display;

use crate::parser::ast::{
  expression::Expression, type_expr::TypeExpression, var::var_decl::LocalDecl,
};

#[derive(Copy, Clone, Debug)]
pub enum Mutability {
  Mutable,
  Immutable,
}

#[derive(Clone, Debug)]
pub struct BindStatement {
  mutability: Mutability,
  var: LocalDecl,
  var_type: Option<TypeExpression>,
  expr: Expression,
}

impl BindStatement {
  pub fn new_let(var: LocalDecl, var_type: Option<TypeExpression>, expr: Expression) -> Self {
    Self {
      mutability: Mutability::Immutable,
      var,
      var_type,
      expr,
    }
  }

  pub fn new_mut(var: LocalDecl, var_type: Option<TypeExpression>, expr: Expression) -> Self {
    Self {
      mutability: Mutability::Mutable,
      var,
      var_type,
      expr,
    }
  }

  pub fn mutability(&self) -> &Mutability {
    &self.mutability
  }

  pub fn var(&self) -> &LocalDecl {
    &self.var
  }

  pub fn var_type(&self) -> Option<&TypeExpression> {
    self.var_type.as_ref()
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
      type_expr::TypeExpression,
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
      mutability: pat!(Mutability::Immutable),
      var: local_decl(var_matcher),
      var_type: none(),
      expr: expr_matcher,
    })))
  }

  pub fn let_stmt_with_type<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    type_matcher: impl Matcher<&'a TypeExpression>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      mutability: pat!(Mutability::Immutable),
      var: local_decl(var_matcher),
      var_type: some(type_matcher),
      expr: expr_matcher,
    })))
  }

  pub fn mut_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      mutability: pat!(Mutability::Mutable),
      var: local_decl(var_matcher),
      var_type: none(),
      expr: expr_matcher,
    })))
  }

  pub fn mut_stmt_with_type<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    type_matcher: impl Matcher<&'a TypeExpression>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Bind(pat!(BindStatement {
      mutability: pat!(Mutability::Mutable),
      var: local_decl(var_matcher),
      var_type: some(type_matcher),
      expr: expr_matcher,
    })))
  }
}
