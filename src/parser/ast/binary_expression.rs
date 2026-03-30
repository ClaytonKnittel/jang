use crate::parser::ast::expression::Expression;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
  lhs: Box<Expression>,
  rhs: Box<Expression>,
  op: BinaryOp,
}

impl BinaryExpression {
  pub fn lhs(&self) -> &Expression {
    &self.lhs
  }

  pub fn rhs(&self) -> &Expression {
    &self.rhs
  }

  pub fn op(&self) -> BinaryOp {
    self.op
  }
}

impl BinaryExpression {
  pub fn new(
    lhs: impl Into<Box<Expression>>,
    rhs: impl Into<Box<Expression>>,
    op: BinaryOp,
  ) -> Self {
    Self {
      lhs: lhs.into(),
      rhs: rhs.into(),
      op,
    }
  }
}

#[cfg(test)]
pub mod matchers {
  use std::ops::Deref;

  use crate::parser::ast::{
    binary_expression::{BinaryExpression, BinaryOp},
    expression::Expression,
  };
  use googletest::prelude::*;

  pub fn binary_expression<'a>(
    lhs_matcher: impl Matcher<&'a Expression>,
    op: &BinaryOp,
    rhs_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::BinaryExpression(pat!(BinaryExpression {
      lhs: result_of!(Box::deref, lhs_matcher),
      rhs: result_of!(Box::deref, rhs_matcher),
      op: eq(op)
    })))
  }
}
