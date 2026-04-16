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
