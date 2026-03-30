use crate::parser::{
  ast::{
    block::{NonRetBlock, RetBlock},
    expression::Expression,
  },
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub enum NonRetStatement {
  Let(LetStatement),
  Block(NonRetBlock),
}

impl From<LetStatement> for NonRetStatement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
  }
}

impl From<NonRetBlock> for NonRetStatement {
  fn from(value: NonRetBlock) -> Self {
    Self::Block(value)
  }
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

#[derive(Clone, Debug)]
pub enum RetStatement {
  Ret(RetExpression),
  Block(Box<RetBlock>),
}

impl From<RetExpression> for RetStatement {
  fn from(value: RetExpression) -> Self {
    Self::Ret(value)
  }
}

impl<T: Into<Box<RetBlock>>> From<T> for RetStatement {
  fn from(value: T) -> Self {
    Self::Block(value.into())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      expression::Expression,
      statement::{LetStatement, NonRetStatement, RetExpression, RetStatement},
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn let_statement<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::Let(pat!(LetStatement {
      var: var_matcher,
      expr: expr_matcher
    })))
  }

  pub fn ret_expression<'a>(
    expected: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a RetStatement> {
    pat!(RetStatement::Ret(pat!(RetExpression { expr: expected })))
  }
}
