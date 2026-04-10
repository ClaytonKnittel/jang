use std::fmt::Display;

use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct AssignmentStatement {
  kind: AssignmentKind,
  var: Ident,
  expr: Expression,
}

#[derive(Clone, Debug)]
pub enum AssignmentKind {
  Declaration(Mutability),
  Rebind,
}

#[derive(Clone, Debug)]
pub enum Mutability {
  Mutable,
  Immutable,
}

impl AssignmentStatement {
  pub fn new_let(var: Ident, expr: Expression) -> Self {
    Self {
      var,
      expr,
      kind: AssignmentKind::Declaration(Mutability::Immutable),
    }
  }

  pub fn new_mut(var: Ident, expr: Expression) -> Self {
    Self {
      var,
      expr,
      kind: AssignmentKind::Declaration(Mutability::Mutable),
    }
  }

  pub fn new_rebind(var: Ident, expr: Expression) -> Self {
    Self {
      var,
      expr,
      kind: AssignmentKind::Rebind,
    }
  }

  pub fn var(&self) -> &Ident {
    &self.var
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }

  pub fn kind(&self) -> &AssignmentKind {
    &self.kind
  }
}

impl Display for AssignmentStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "let {} = {}", self.var, self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      assignment_statement::{AssignmentKind, AssignmentStatement, Mutability},
      expression::Expression,
      statement::Statement,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn assignment_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
    kind_matcher: impl Matcher<&'a AssignmentKind>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Assign(pat!(AssignmentStatement {
      var: var_matcher,
      expr: expr_matcher,
      kind: kind_matcher,
    })))
  }

  pub fn let_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    assignment_stmt(
      var_matcher,
      expr_matcher,
      pat!(AssignmentKind::Declaration(pat!(Mutability::Immutable))),
    )
  }

  pub fn mut_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    assignment_stmt(
      var_matcher,
      expr_matcher,
      pat!(AssignmentKind::Declaration(pat!(Mutability::Mutable))),
    )
  }

  pub fn rebind_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    assignment_stmt(var_matcher, expr_matcher, pat!(AssignmentKind::Rebind))
  }
}
