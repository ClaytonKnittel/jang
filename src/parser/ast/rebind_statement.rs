use std::fmt::Display;

use crate::parser::ast::{expression::Expression, name_ref_expression::NameRefExpression};

#[derive(Clone, Debug)]
pub struct RebindStatement {
  var: NameRefExpression,
  expr: Expression,
}

impl RebindStatement {
  pub fn new(var: NameRefExpression, expr: Expression) -> Self {
    Self { var, expr }
  }

  pub fn var(&self) -> &NameRefExpression {
    &self.var
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }
}

impl Display for RebindStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} = {}", self.var, self.expr)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{
      expression::Expression, name_ref_expression::matchers::name_ref_expr,
      rebind_statement::RebindStatement, statement::Statement,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn rebind_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Rebind(pat!(RebindStatement {
      var: name_ref_expr(var_matcher),
      expr: expr_matcher,
    })))
  }
}
