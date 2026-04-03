use std::fmt::Display;

use crate::parser::ast::{call_expression::CallExpression, if_statement::IfExpression};

#[derive(Clone, Debug)]
pub enum StandaloneExpression {
  CallExpression(CallExpression),
  IfExpression(Box<IfExpression>),
}

impl From<CallExpression> for StandaloneExpression {
  fn from(value: CallExpression) -> Self {
    Self::CallExpression(value)
  }
}

impl From<Box<IfExpression>> for StandaloneExpression {
  fn from(value: Box<IfExpression>) -> Self {
    Self::IfExpression(value)
  }
}

impl Display for StandaloneExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CallExpression(call_expr) => write!(f, "{call_expr}"),
      Self::IfExpression(if_expr) => write!(f, "{if_expr}"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    call_expression::CallExpression, expression::Expression,
    standalone_expression::StandaloneExpression, statement::NonRetStatement,
  };
  use googletest::prelude::*;

  pub fn call_expression<'a>(
    matcher: impl Matcher<&'a CallExpression>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::StandaloneExpression(pat!(
      StandaloneExpression::CallExpression(matcher)
    )))
  }

  pub fn call_expr_stmt<'a>(
    call_expr_matcher: impl Matcher<&'a CallExpression>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::StandaloneExpression(pat!(
      StandaloneExpression::CallExpression(call_expr_matcher)
    )))
  }
}
