use std::fmt::Display;

use crate::parser::ast::{
  block::NonRetBlock, call_expression::CallExpression, if_statement::IfStatement,
  let_statement::LetStatement,
};

#[derive(Clone, Debug)]
pub enum NonRetStatement {
  Let(LetStatement),
  CallStatement(CallExpression),
  IfStatement(IfStatement),
  Block(NonRetBlock),
}

impl From<LetStatement> for NonRetStatement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
  }
}

impl From<CallExpression> for NonRetStatement {
  fn from(value: CallExpression) -> Self {
    Self::CallStatement(value)
  }
}

impl From<IfStatement> for NonRetStatement {
  fn from(value: IfStatement) -> Self {
    Self::IfStatement(value)
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
      Self::CallStatement(call_stmt) => write!(f, "{call_stmt}"),
      Self::IfStatement(if_stmt) => write!(f, "{if_stmt}"),
      Self::Block(block) => write!(f, "{block}"),
    }
  }
}
