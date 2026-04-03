use std::fmt::Display;

use crate::parser::ast::{
  block::NonRetBlock, let_statement::LetStatement, standalone_expression::StandaloneExpression,
};

#[derive(Clone, Debug)]
pub enum NonRetStatement {
  Let(LetStatement),
  StandaloneExpression(StandaloneExpression),
  Block(NonRetBlock),
}

impl From<LetStatement> for NonRetStatement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
  }
}

impl From<StandaloneExpression> for NonRetStatement {
  fn from(value: StandaloneExpression) -> Self {
    Self::StandaloneExpression(value)
  }
}

impl From<NonRetBlock> for NonRetStatement {
  fn from(value: NonRetBlock) -> Self {
    Self::Block(value)
  }
}

impl Display for NonRetStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Let(let_stmt) => write!(f, "{let_stmt}"),
      Self::StandaloneExpression(expr) => write!(f, "{expr}"),
      Self::Block(block) => write!(f, "{block}"),
    }
  }
}
