use std::fmt::Display;

use crate::parser::ast::{block::Block, expression::Expression};

#[derive(Clone, Debug)]
pub enum ElseClause {
  None,
  Else(Block),
  ElseIf(Box<IfStatement>),
}

#[derive(Clone, Debug)]
pub struct IfStatement {
  condition: Box<Expression>,
  body: Block,
  else_clause: ElseClause,
}

impl IfStatement {
  pub fn new(condition: impl Into<Box<Expression>>, body: Block) -> Self {
    Self {
      condition: condition.into(),
      body,
      else_clause: ElseClause::None,
    }
  }

  pub fn new_with_else(
    condition: impl Into<Box<Expression>>,
    body: Block,
    else_block: Block,
  ) -> Self {
    Self {
      condition: condition.into(),
      body,
      else_clause: ElseClause::Else(else_block),
    }
  }

  pub fn new_with_else_if(
    condition: impl Into<Box<Expression>>,
    body: Block,
    else_if: impl Into<Box<IfStatement>>,
  ) -> Self {
    Self {
      condition: condition.into(),
      body,
      else_clause: ElseClause::ElseIf(else_if.into()),
    }
  }

  pub fn condition(&self) -> &Expression {
    &self.condition
  }

  pub fn body(&self) -> &Block {
    &self.body
  }

  pub fn else_clause(&self) -> &ElseClause {
    &self.else_clause
  }
}

impl Display for IfStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "if {} {}", self.condition(), self.body())?;
    match self.else_clause() {
      ElseClause::None => {}
      ElseClause::Else(else_block) => write!(f, " else {else_block}")?,
      ElseClause::ElseIf(else_if) => write!(f, " else {else_if}")?,
    }
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{
    block::Block,
    expression::Expression,
    if_statement::{ElseClause, IfStatement},
    statement::NonRetStatement,
  };
  use googletest::prelude::*;

  pub fn if_clause<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a IfStatement> {
    pat!(IfStatement {
      condition: result_of!(Box::as_ref, cond_matcher),
      body: body_matcher,
      else_clause: pat!(ElseClause::None),
    })
  }

  pub fn if_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::IfStatement(if_clause(
      cond_matcher,
      body_matcher
    )))
  }

  pub fn if_else_clause<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
    else_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a IfStatement> {
    pat!(IfStatement {
      condition: result_of!(Box::as_ref, cond_matcher),
      body: body_matcher,
      else_clause: pat!(ElseClause::Else(else_matcher)),
    })
  }

  pub fn if_else_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
    else_matcher: impl Matcher<&'a Block>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::IfStatement(if_else_clause(
      cond_matcher,
      body_matcher,
      else_matcher
    )))
  }

  pub fn if_else_if_clause<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
    else_if_matcher: impl Matcher<&'a IfStatement>,
  ) -> impl Matcher<&'a IfStatement> {
    pat!(IfStatement {
      condition: result_of!(Box::as_ref, cond_matcher),
      body: body_matcher,
      else_clause: pat!(ElseClause::ElseIf(result_of!(Box::as_ref, else_if_matcher))),
    })
  }

  pub fn if_else_if_statement<'a>(
    cond_matcher: impl Matcher<&'a Expression>,
    body_matcher: impl Matcher<&'a Block>,
    else_if_matcher: impl Matcher<&'a IfStatement>,
  ) -> impl Matcher<&'a NonRetStatement> {
    pat!(NonRetStatement::IfStatement(if_else_if_clause(
      cond_matcher,
      body_matcher,
      else_if_matcher
    )))
  }
}
