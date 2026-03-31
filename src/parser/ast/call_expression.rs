use crate::parser::{
  ast::{expression::Expression, expression_list::ExpressionList},
  token::ident::Ident,
};

#[derive(Clone, Debug)]
pub struct CallExpression {
  name: Ident,
  argument_list: ExpressionList,
}

impl CallExpression {
  pub fn new(name: Ident, argument_list: impl Into<ExpressionList>) -> Self {
    Self {
      name,
      argument_list: argument_list.into(),
    }
  }

  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn argument_list(&self) -> &[Expression] {
    self.argument_list.expressions()
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::{call_expression::CallExpression, expression::Expression},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn call_expr_name<'a>(name_matcher: impl Matcher<&'a Ident>) -> impl Matcher<&'a Expression> {
    pat!(Expression::CallExpression(property!(
      &CallExpression.name(),
      name_matcher
    )))
  }

  pub fn call_expr_args<'a>(
    args_matcher: impl Matcher<&'a [Expression]>,
  ) -> impl Matcher<&'a Expression> {
    pat!(Expression::CallExpression(property!(
      &CallExpression.argument_list(),
      args_matcher
    )))
  }
}
