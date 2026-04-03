use std::fmt::Display;

use crate::parser::ast::expression::Expression;

#[derive(Clone, Debug)]
pub struct RetExpression {
  expr: Expression,
}

impl RetExpression {
  pub fn new(expr: Expression) -> Self {
    Self { expr }
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

impl Display for RetExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ret {}", self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    expression::Expression, ret_statement::RetExpression, statement::Statement,
  };
  use googletest::prelude::*;

  pub fn ret_statement<'a>(expected: impl Matcher<&'a Expression>) -> impl Matcher<&'a Statement> {
    pat!(Statement::Ret(pat!(RetExpression { expr: expected })))
  }
}
