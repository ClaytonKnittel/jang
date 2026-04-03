use std::fmt::Display;

use crate::parser::ast::expression::Expression;

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

impl Display for RetStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ret {}", self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    expression::Expression, ret_statement::RetStatement, statement::Statement,
  };
  use googletest::prelude::*;

  pub fn ret_statement<'a>(expected: impl Matcher<&'a Expression>) -> impl Matcher<&'a Statement> {
    pat!(Statement::Ret(pat!(RetStatement { expr: expected })))
  }
}
