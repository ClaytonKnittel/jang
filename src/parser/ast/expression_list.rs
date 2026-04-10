use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::ast::expression::Expression;

#[derive(Clone, Debug, Builder)]
pub struct ExpressionList {
  #[vec]
  expressions: Vec<Expression>,
}

impl ExpressionList {
  pub fn expressions(&self) -> &[Expression] {
    &self.expressions
  }
}

impl Display for ExpressionList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut first = true;
    for expr in &self.expressions {
      if first {
        first = false;
      } else {
        write!(f, ", ")?;
      }
      write!(f, "{expr}")?;
    }

    Ok(())
  }
}
