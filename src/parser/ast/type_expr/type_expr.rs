use std::fmt::Display;

use cknittel_util::builder::Builder;
use itertools::Itertools;

use crate::parser::{
  ast::{structured_type_decl::StructuredTypeDecl, type_expr::primitive::PrimitiveType},
  token::ident::Ident,
};

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
        "({})",
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
pub struct InlineFn {
  args: TypeExpressionList,
  return_type: Box<TypeExpression>,
}

impl InlineFn {
  pub fn args(&self) -> &[TypeExpression] {
    &self.args.expressions()
  }

  pub fn return_type(&self) -> &TypeExpression {
    &self.return_type
  }
}

impl Display for InlineFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} -> {}", self.args, self.return_type)
  }
}

#[derive(Clone, Debug)]
pub enum TypeExpressionVariant {
  Unit,
  Primitive(PrimitiveType),
  Named(Ident),
  AnonymousStruct(StructuredTypeDecl),
  InlineFn(InlineFn),
}

impl Display for TypeExpressionVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unit => write!(f, "unit"),
      Self::Primitive(primitive) => write!(f, "{primitive}"),
      Self::Named(ident) => write!(f, "{ident}"),
      Self::AnonymousStruct(struct_decl) => write!(f, "{struct_decl}"),
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

  pub fn new_primitive(primitive: PrimitiveType) -> Self {
    Self {
      variant: TypeExpressionVariant::Primitive(primitive),
    }
  }

  pub fn new_named(ident: Ident) -> Self {
    Self {
      variant: TypeExpressionVariant::Named(ident),
    }
  }

  pub fn new_anon_struct(type_decl: StructuredTypeDecl) -> Self {
    Self {
      variant: TypeExpressionVariant::AnonymousStruct(type_decl),
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

  pub fn variant(&self) -> &TypeExpressionVariant {
    &self.variant
  }
}

impl Display for TypeExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.variant)
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use crate::parser::{ast::structured_type_decl::StructuredTypeField, token::ident::Ident};
  use googletest::prelude::*;

  pub fn unit_type_expr<'a>() -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::Unit)
    })
  }

  pub fn primitive_type_expr(primitive: &PrimitiveType) -> impl Matcher<&TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::Primitive(eq(primitive)))
    })
  }

  pub fn named_type_expr<'a>(
    name_matcher: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::Named(name_matcher))
    })
  }

  pub fn structured_type<'a>(
    field_matchers: impl Matcher<&'a [StructuredTypeField]>,
  ) -> impl Matcher<&'a TypeExpression> {
    pat!(TypeExpression {
      variant: pat!(TypeExpressionVariant::AnonymousStruct(property!(
        &StructuredTypeDecl.fields(),
        field_matchers
      ))),
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
