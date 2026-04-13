use std::fmt::Display;

use crate::parser::ast::{expression::Expression, var_ref::VarRef};

#[derive(Clone, Debug)]
pub struct RebindStatement {
  var: VarRef,
  expr: Expression,
}

impl RebindStatement {
  pub fn new(var: VarRef, expr: Expression) -> Self {
    Self { var, expr }
  }

  pub fn var(&self) -> &VarRef {
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
      expression::Expression, rebind_statement::RebindStatement, statement::Statement,
      var_ref::matchers::var_ref,
    },
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn rebind_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Rebind(pat!(RebindStatement {
      var: var_ref(var_matcher),
      expr: expr_matcher,
    })))
  }
}
