use crate::parser::ast::expression::Expression;

#[derive(Default)]
pub struct ExpressionListBuilder {
  expressions: Vec<Expression>,
}

impl ExpressionListBuilder {
  pub fn push(mut self, expression: Expression) -> Self {
    self.expressions.push(expression);
    self
  }

  pub fn build(self) -> ExpressionList {
    ExpressionList {
      expressions: self.expressions,
    }
  }
}

impl From<ExpressionListBuilder> for ExpressionList {
  fn from(value: ExpressionListBuilder) -> Self {
    value.build()
  }
}

#[derive(Clone, Debug)]
pub struct ExpressionList {
  expressions: Vec<Expression>,
}

impl ExpressionList {
  pub fn expressions(&self) -> &[Expression] {
    &self.expressions
  }
}
