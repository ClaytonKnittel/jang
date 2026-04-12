use std::fmt::Display;

use crate::parser::{ast::ids::AstExpressionId, token::literal::Literal};

#[derive(Clone, Debug)]
pub struct LiteralExpression {
  id: AstExpressionId,
  literal: Literal,
}

impl LiteralExpression {
  pub fn new(id: AstExpressionId, literal: Literal) -> Self {
    Self { id, literal }
  }

  pub fn literal(&self) -> &Literal {
    &self.literal
  }

  pub fn id(&self) -> AstExpressionId {
    self.id
  }
}

impl Display for LiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.literal)
  }
}

#[cfg(test)]
pub mod matchers {
  use googletest::prelude::*;

  use crate::parser::{
    ast::{expression::Expression, literal_expression::LiteralExpression},
    token::literal::Literal,
  };

  pub fn literal_expression<'a>(
    matcher: impl Matcher<&'a Literal>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::Literal(pat!(LiteralExpression {
      id: anything(),
      literal: matcher,
    })))
  }
}
