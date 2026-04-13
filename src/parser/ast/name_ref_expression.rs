use std::fmt::Display;

use crate::parser::{ast::id::def::AstNameRefExpressionId, token::ident::Ident};

#[derive(Clone, Debug)]
pub struct NameRefExpression {
  name_ref_id: AstNameRefExpressionId,
  ident: Ident,
}

impl NameRefExpression {
  pub(super) fn new(name_ref_id: AstNameRefExpressionId, ident: Ident) -> Self {
    Self { name_ref_id, ident }
  }

  pub fn name(&self) -> &Ident {
    &self.ident
  }

  pub fn name_ref_id(&self) -> AstNameRefExpressionId {
    self.name_ref_id
  }
}

impl Display for NameRefExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.ident)
  }
}

#[cfg(test)]
pub mod matchers {
  use googletest::prelude::*;

  use crate::parser::{
    ast::{
      expression::{Expression, ExpressionVariant, matchers::expr_variant},
      name_ref_expression::NameRefExpression,
    },
    token::ident::Ident,
  };

  pub fn name_ref_expr<'a>(
    matcher: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a NameRefExpression> {
    property!(&NameRefExpression.name(), matcher)
  }

  pub fn ident_expression<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a Expression> {
    expr_variant(pat!(ExpressionVariant::NameRef(pat!(NameRefExpression {
      name_ref_id: anything(),
      ident: matcher,
    }))))
  }
}
