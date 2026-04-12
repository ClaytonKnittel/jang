use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::ast::{
  binary_expression::BinaryExpression, call_expression::CallExpression,
  dot_expression::DotExpression, ids::AstExpressionId, literal_expression::LiteralExpression,
  name_ref_expression::NameRefExpression, unary_experssion::UnaryExpression,
};

#[derive(Clone, Debug, FromVariants)]
pub enum Expression {
  Literal(LiteralExpression),
  NameRef(NameRefExpression),
  BinaryExpression(BinaryExpression),
  UnaryExpression(UnaryExpression),
  CallExpression(CallExpression),
  DotExpression(DotExpression),
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(literal) => write!(f, "{literal}"),
      Self::NameRef(ident) => write!(f, "{ident}"),
      Self::BinaryExpression(binary_expr) => write!(f, "({binary_expr})"),
      Self::UnaryExpression(unary_expr) => write!(f, "{unary_expr}"),
      Self::CallExpression(call_expr) => write!(f, "{call_expr}"),
      Self::DotExpression(dot_expr) => write!(f, "{dot_expr}"),
    }
  }
}

impl Expression {
  pub fn id(&self) -> AstExpressionId {
    match self {
      Self::Literal(literal) => literal.id(),
      Self::NameRef(ident) => ident.id(),
      Self::BinaryExpression(binary_expr) => binary_expr.id(),
      Self::UnaryExpression(unary_expr) => unary_expr.id(),
      Self::CallExpression(call_expr) => call_expr.id(),
      Self::DotExpression(dot_expr) => dot_expr.id(),
    }
  }
}
