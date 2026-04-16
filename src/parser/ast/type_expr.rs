use std::fmt::Display;

use cknittel_util::builder::Builder;
use itertools::Itertools;

use crate::parser::token::ident::Ident;

#[derive(Clone, Debug, Builder)]
pub struct TypeExpressionList {
  #[vec]
  expressions: Vec<TypeExpression>,
}

impl TypeExpressionList {
  pub fn empty() -> Self {
    Self {
      expressions: Vec::new(),
    }
  }

  pub fn expressions(&self) -> &[TypeExpression] {
    &self.expressions
  }
}

impl Display for TypeExpressionList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.expressions().is_empty() {
      write!(f, "()")
    } else {
      write!(
        f,
        "{}",
        self
          .expressions()
          .iter()
          .map(|arg| format!("{arg}"))
          .join(",")
      )
    }
  }
}

#[derive(Clone, Debug)]
struct InlineFn {
  args: TypeExpressionList,
  return_type: Box<TypeExpression>,
}

impl Display for InlineFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} -> {}", self.args, self.return_type)
  }
}

#[derive(Clone, Debug)]
enum TypeExpressionVariant {
  Unit,
  Named(Ident),
  InlineFn(InlineFn),
}

impl Display for TypeExpressionVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unit => write!(f, "unit"),
      Self::Named(ident) => write!(f, "{ident}"),
      Self::InlineFn(inline_fn) => write!(f, "{inline_fn}"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypeExpression {
  variant: TypeExpressionVariant,
}

impl TypeExpression {
  pub fn new_unit() -> Self {
    Self {
      variant: TypeExpressionVariant::Unit,
    }
  }

  pub fn new_named(ident: Ident) -> Self {
    Self {
      variant: TypeExpressionVariant::Named(ident),
    }
  }

  pub fn new_inline_fn(
    args: TypeExpressionList,
    return_type: impl Into<Box<TypeExpression>>,
  ) -> Self {
    Self {
      variant: TypeExpressionVariant::InlineFn(InlineFn {
        args,
        return_type: return_type.into(),
      }),
    }
  }
}

impl Display for TypeExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.variant)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::{
    ast::type_expr::{InlineFn, TypeExpression, TypeExpressionList, TypeExpressionVariant},
    token::ident::Ident,
  };
  use googletest::prelude::*;

  pub fn unit_type_expr<'a>() -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::Unit)
    })
  }

  pub fn named_type_expr<'a>(
    expected: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::Named(expected))
    })
  }

  fn type_expr_list<'a>(
    expressions_matcher: impl Matcher<&'a [TypeExpression]>,
  ) -> impl Matcher<&'a TypeExpressionList> {
    property!(&TypeExpressionList.expressions(), expressions_matcher)
  }

  pub fn fn_type_expr<'a>(
    args_matcher: impl Matcher<&'a [TypeExpression]>,
    return_type_matcher: impl Matcher<&'a TypeExpression>,
  ) -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::InlineFn(pat!(InlineFn {
        args: type_expr_list(args_matcher),
        return_type: result_of!(Box::as_ref, return_type_matcher),
      })))
    })
  }
}
