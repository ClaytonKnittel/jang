use std::fmt::Display;

use crate::parser::ast::{expression::Expression, ids::AstExpressionId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
  LogicalNot,
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::LogicalNot => '!',
      }
    )
  }
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
  id: AstExpressionId,
  expr: Box<Expression>,
  op: UnaryOp,
}

impl UnaryExpression {
  pub fn expr(&self) -> &Expression {
    &self.expr
  }

  pub fn op(&self) -> UnaryOp {
    self.op
  }

  pub fn id(&self) -> AstExpressionId {
    self.id
  }
}

impl UnaryExpression {
  pub fn new(id: AstExpressionId, expr: impl Into<Box<Expression>>, op: UnaryOp) -> Self {
    Self {
      id,
      expr: expr.into(),
      op,
    }
  }
}

impl Display for UnaryExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.op, self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use std::ops::Deref;

  use crate::parser::ast::{
    expression::Expression,
    unary_experssion::{UnaryExpression, UnaryOp},
  };
  use googletest::prelude::*;

  pub fn unary_exp<'a>(
    op: &UnaryOp,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::UnaryExpression(pat!(UnaryExpression {
      id: anything(),
      expr: result_of!(Box::deref, expr_matcher),
      op: eq(op)
    })))
  }

  pub fn logical_not_exp<'a>(
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Expression> {
    unary_exp(&UnaryOp::LogicalNot, expr_matcher)
  }
}
