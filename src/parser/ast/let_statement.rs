use std::fmt::Display;

use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct LetStatement {
  var: Ident,
  expr: Expression,
}

impl LetStatement {
  pub fn new(var: Ident, expr: Expression) -> Self {
    Self { var, expr }
  }

  pub fn var(&self) -> &Ident {
    &self.var
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

impl Display for LetStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "let {} = {}", self.var, self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{expression::Expression, let_statement::LetStatement, statement::Statement},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn let_statement<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Let(pat!(LetStatement {
      var: var_matcher,
      expr: expr_matcher
    })))
  }
}
