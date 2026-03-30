use crate::parser::ast::expression::Expression;

#[derive(Clone, Debug)]
pub enum Statement {
  Ret(RetStatement),
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
  use crate::parser::ast::{
    expression::Expression,
    statement::{RetStatement, Statement},
  };
  use googletest::prelude::*;

  pub fn ret_statement<'a>(expected: impl Matcher<&'a Expression>) -> impl Matcher<&'a Statement> {
    pat!(Statement::Ret(pat!(RetStatement { expr: expected })))
  }
}
