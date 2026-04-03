use std::fmt::Display;

use crate::parser::ast::{block::Block, expression::Expression};

#[derive(Clone, Debug)]
pub struct IfExpression {
  condition: Expression,
  body: Block,
  else_expr: Option<Expression>,
}

impl IfExpression {
  pub fn new(condition: Expression, body: Block) -> Self {
    Self {
      condition,
      body,
      else_expr: None,
    }
  }

  pub fn condition(&self) -> &Expression {
    &self.condition
  }

  pub fn body(&self) -> &Block {
    &self.body
  }

  pub fn else_expr(&self) -> Option<&Expression> {
    self.else_expr.as_ref()
  }
}

impl Display for IfExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!();
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    block::Block, expression::Expression, if_statement::IfExpression,
    standalone_expression::StandaloneExpression, statement::NonRetStatement,
  };
  use googletest::prelude::*;

  pub fn if_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::StandaloneExpression(pat!(
      StandaloneExpression::IfExpression(result_of!(
        Box::as_ref,
        pat!(IfExpression {
          condition: cond_matcher,
          body: body_matcher,
          else_expr: none()
        })
      ))
    )))
  }
}
