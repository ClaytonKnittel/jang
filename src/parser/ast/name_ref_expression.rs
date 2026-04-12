use std::fmt::Display;

use crate::parser::{
  ast::ids::{AstExpressionId, AstNameRefExpressionId},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct NameRefExpression {
  id: AstExpressionId,
  name_ref_id: AstNameRefExpressionId,
  ident: Ident,
}

impl NameRefExpression {
  pub fn new(id: AstExpressionId, name_ref_id: AstNameRefExpressionId, ident: Ident) -> Self {
    Self {
      id,
      name_ref_id,
      ident,
    }
  }

  pub fn ident(&self) -> &Ident {
    &self.ident
  }

  pub fn id(&self) -> AstExpressionId {
    self.id
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
    ast::{expression::Expression, name_ref_expression::NameRefExpression},
    token::ident::Ident,
  };

  pub fn ident_expression<'a>(matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a Expression> {
    pat!(Expression::NameRef(pat!(NameRefExpression {
      id: anything(),
      name_ref_id: anything(),
      ident: matcher,
    })))
  }
}
