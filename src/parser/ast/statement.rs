use std::fmt::Display;

use crate::parser::ast::{
  block::Block, call_expression::CallExpression, if_statement::IfStatement,
  let_statement::LetStatement, loop_statement::LoopStatement, ret_statement::RetStatement,
};

#[derive(Clone, Debug)]
pub enum Statement {
  Let(LetStatement),
  Ret(RetStatement),
  CallStatement(CallExpression),
  IfStatement(IfStatement),
  LoopStatement(LoopStatement),
  Block(Block),
}

impl From<LetStatement> for Statement {
  fn from(value: LetStatement) -> Self {
    Self::Let(value)
  }
}

impl From<RetStatement> for Statement {
  fn from(value: RetStatement) -> Self {
    Self::Ret(value)
  }
}

impl From<CallExpression> for Statement {
  fn from(value: CallExpression) -> Self {
    Self::CallStatement(value)
  }
}

impl From<IfStatement> for Statement {
  fn from(value: IfStatement) -> Self {
    Self::IfStatement(value)
  }
}

impl From<LoopStatement> for NonRetStatement {
  fn from(value: LoopStatement) -> Self {
    Self::LoopStatement(value)
  }
}

impl From<LoopStatement> for NonRetStatement {
  fn from(value: LoopStatement) -> Self {
    Self::LoopStatement(value)
  }
}

impl From<Block> for Statement {
  fn from(value: Block) -> Self {
    Self::Block(value)
  }
}

impl Display for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Let(let_stmt) => write!(f, "{let_stmt}"),
      Self::Ret(ret_stmt) => write!(f, "{ret_stmt}"),
      Self::CallStatement(call_stmt) => write!(f, "{call_stmt}"),
      Self::IfStatement(if_stmt) => write!(f, "{if_stmt}"),
      Self::LoopStatement(loop_stmt) => write!(f, "{loop_stmt}"),
      Self::Block(block) => write!(f, "{block}"),
    }
  }
}
