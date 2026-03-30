use crate::parser::{ast::expression::Expression, token::ident::Ident};

#[derive(Clone, Debug)]
pub enum NonRetStatement {
  Let(LetStatement),
}

impl From<LetStatement> for NonRetStatement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
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
      statement::{LetStatement, NonRetStatement, RetStatement},
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

  pub fn ret_statement<'a>(
    expected: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a RetStatement> {
    pat!(RetStatement { expr: expected })
  }
}
