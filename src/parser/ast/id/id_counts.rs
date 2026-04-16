#[derive(Clone, Debug)]
pub struct AstIdCounts {
  pub(super) global_decl_count: usize,
  pub(super) local_decl_count: usize,
  pub(super) expression_count: usize,
}

impl AstIdCounts {
  pub fn global_decl_count(&self) -> usize {
    self.global_decl_count
  }

  pub fn local_decl_count(&self) -> usize {
    self.local_decl_count
  }

  pub fn expression_count(&self) -> usize {
    self.expression_count
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::id::id_counts::AstIdCounts;
  use googletest::prelude::*;

  pub fn ast_global_decl_id_count<'a>(count: impl Matcher<usize>) -> impl Matcher<&'a AstIdCounts> {
    property!(&AstIdCounts.global_decl_count(), count)
  }

  pub fn ast_local_decl_id_count<'a>(count: impl Matcher<usize>) -> impl Matcher<&'a AstIdCounts> {
    property!(&AstIdCounts.local_decl_count(), count)
  }

  pub fn ast_expression_id_count<'a>(count: impl Matcher<usize>) -> impl Matcher<&'a AstIdCounts> {
    property!(&AstIdCounts.expression_count(), count)
  }
}
