use std::fmt::Display;

use crate::parser::ast::{expression::Expression, expression_list::ExpressionList};

#[derive(Clone, Debug)]
pub struct CallExpression {
  target: Box<Expression>,
  argument_list: ExpressionList,
}

impl CallExpression {
  pub fn new(target: impl Into<Box<Expression>>, argument_list: impl Into<ExpressionList>) -> Self {
    Self {
      target: target.into(),
      argument_list: argument_list.into(),
    }
  }

  pub fn target(&self) -> &Expression {
    &self.target
  }

  pub fn argument_list(&self) -> &[Expression] {
    self.argument_list.expressions()
  }
}

impl Display for CallExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}({})", self.target, self.argument_list)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    call_expression::CallExpression,
    expression::{Expression, ExpressionVariant, matchers::expr_variant},
    statement::Statement,
  };
  use googletest::prelude::*;

  pub fn call_statement<'a>(
    call_expr_matcher: impl Matcher<&'a CallExpression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::CallStatement(call_expr_matcher))
  }

  pub fn call_expression<'a>(
    matcher: impl Matcher<&'a CallExpression>,
  ) -> impl Matcher<&'a Expression> {
    expr_variant(pat!(ExpressionVariant::CallExpression(matcher)))
  }

  pub fn call_expr_target<'a>(
    target_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a CallExpression> {
    property!(&CallExpression.target(), target_matcher)
  }

  pub fn call_expr_args<'a>(
    args_matcher: impl Matcher<&'a [Expression]>,
  ) -> impl Matcher<&'a CallExpression> {
    property!(&CallExpression.argument_list(), args_matcher)
  }
}
