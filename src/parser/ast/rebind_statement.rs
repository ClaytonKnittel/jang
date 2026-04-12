use std::fmt::Display;

use crate::parser::{
  ast::{expression::Expression, ids::AstNameRefExpressionId},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct RebindStatement {
  name_ref_id: AstNameRefExpressionId,
  var: Ident,
  expr: Expression,
}

impl RebindStatement {
  pub fn new(name_ref_id: AstNameRefExpressionId, var: Ident, expr: Expression) -> Self {
    Self {
      name_ref_id,
      var,
      expr,
    }
  }

  pub fn var(&self) -> &Ident {
    &self.var
  }

  pub fn expr(&self) -> &Expression {
    &self.expr
  }

  pub fn name_ref_id(&self) -> AstNameRefExpressionId {
    self.name_ref_id
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
    ast::{expression::Expression, rebind_statement::RebindStatement, statement::Statement},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn rebind_stmt<'a>(
    var_matcher: impl Matcher<&'a Ident>,
    expr_matcher: impl Matcher<&'a Expression>,
  ) -> impl Matcher<&'a Statement> {
    pat!(Statement::Rebind(pat!(RebindStatement {
      name_ref_id: anything(),
      var: var_matcher,
      expr: expr_matcher,
    })))
  }
}
