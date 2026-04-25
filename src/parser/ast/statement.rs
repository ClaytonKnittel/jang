use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::ast::{
  bind_statement::BindStatement, block::Block, call_expression::CallExpression,
  if_statement::IfStatement, loop_statement::LoopStatement, rebind_statement::RebindStatement,
  ret_statement::RetStatement,
};

#[derive(Clone, Debug, FromVariants)]
pub enum Statement {
  Bind(BindStatement),
  Rebind(RebindStatement),
  Ret(RetStatement),
  CallStatement(CallExpression),
  IfStatement(IfStatement),
  LoopStatement(LoopStatement),
  Block(Block),
  Break,
}

impl Display for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Bind(bind_statement) => write!(f, "{bind_statement}"),
      Self::Rebind(rebind_stmt) => write!(f, "{rebind_stmt}"),
      Self::Ret(ret_stmt) => write!(f, "{ret_stmt}"),
      Self::CallStatement(call_stmt) => write!(f, "{call_stmt}"),
      Self::IfStatement(if_stmt) => write!(f, "{if_stmt}"),
      Self::LoopStatement(loop_stmt) => write!(f, "{loop_stmt}"),
      Self::Block(block) => write!(f, "{block}"),
      Self::Break => write!(f, "break"),
    }
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    bind_statement::BindStatement, rebind_statement::RebindStatement, ret_statement::RetStatement,
    statement::Statement,
  };
  use googletest::prelude::*;

  pub fn break_statement<'a>() -> impl Matcher<&'a Statement> {
    pat!(Statement::Break)
  }

  impl Statement {
    pub fn as_ret(&self) -> &RetStatement {
      match self {
        Self::Ret(s) => s,
        _ => panic!("Expected RetStatement, got {:?}", self),
      }
    }

    pub fn as_bind(&self) -> &BindStatement {
      match self {
        Self::Bind(s) => s,
        _ => panic!("Expected BindStatement, got {:?}", self),
      }
    }
  }
}
