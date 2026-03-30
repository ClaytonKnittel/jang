use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub enum Statement {
  Let(LetStatement),
  Ret(RetStatement),
}

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

#[derive(Clone, Debug)]
pub struct RetStatement {
  expr: Expression,
}

impl RetStatement {
  pub fn new(expr: Expression) -> Self {
    Self { expr }
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::parser::{
    ast::{
      expression::Expression,
      statement::{LetStatement, RetStatement, Statement},
    },
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

  pub fn ret_statement<'a>(expected: impl Matcher<&'a Expression>) -> impl Matcher<&'a Statement> {
    pat!(Statement::Ret(pat!(RetStatement { expr: expected })))
  }
}
