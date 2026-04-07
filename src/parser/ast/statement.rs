use std::fmt::Display;

use cknittel_util::from_variants::FromVariants;

use crate::parser::ast::{
  block::Block, call_expression::CallExpression, if_statement::IfStatement,
  let_statement::LetStatement, loop_statement::LoopStatement, ret_statement::RetStatement,
};

#[derive(Clone, Debug, FromVariants)]
pub enum Statement {
  Let(LetStatement),
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
      Self::Let(let_stmt) => write!(f, "{let_stmt}"),
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
  use crate::parser::ast::statement::Statement;
  use googletest::prelude::*;

  pub fn break_statement<'a>() -> impl Matcher<&'a Statement> {
    pat!(Statement::Break)
  }
}
