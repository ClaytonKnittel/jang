use crate::parser::ast::{block::Block, expression::Expression};

#[derive(Clone, Debug)]
pub struct IfExpression {
  condition: Expression,
  body: Block,
  else_expr: Option<ElseExpression>,
}

#[derive(Clone, Debug)]
pub enum ElseExpression {
  Else(ElseExpr),
  ElseIf(Box<IfExpression>),
}
