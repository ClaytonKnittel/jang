use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::ast::{
  binary_expression::BinaryExpression, call_expression::CallExpression,
  dot_expression::DotExpression, id::def::AstExpressionId, literal_expression::LiteralExpression,
  unary_experssion::UnaryExpression, var_ref::VarRef,
};

#[derive(Clone, Debug, FromVariants)]
pub enum ExpressionVariant {
  Literal(LiteralExpression),
  VarRef(VarRef),
  BinaryExpression(BinaryExpression),
  UnaryExpression(UnaryExpression),
  CallExpression(CallExpression),
  DotExpression(DotExpression),
}

impl Display for ExpressionVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(literal) => write!(f, "{literal}"),
      Self::VarRef(var_ref) => write!(f, "{var_ref}"),
      Self::BinaryExpression(binary_expr) => write!(f, "({binary_expr})"),
      Self::UnaryExpression(unary_expr) => write!(f, "{unary_expr}"),
      Self::CallExpression(call_expr) => write!(f, "{call_expr}"),
      Self::DotExpression(dot_expr) => write!(f, "{dot_expr}"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Expression {
  id: AstExpressionId,
  variant: ExpressionVariant,
}

impl Expression {
  pub(super) fn new(id: AstExpressionId, variant: ExpressionVariant) -> Self {
    Self { id, variant }
  }

  pub fn id(&self) -> AstExpressionId {
    self.id
  }

  pub fn variant(&self) -> &ExpressionVariant {
    &self.variant
  }
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.variant)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::expression::{Expression, ExpressionVariant};
  use googletest::prelude::*;

  pub fn expr_variant<'a>(
    variant_matcher: impl Matcher<&'a ExpressionVariant>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression {
      id: anything(),
      variant: variant_matcher
    })
  }
}
