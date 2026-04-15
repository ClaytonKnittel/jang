pub trait AstId: Copy {}

pub(super) trait AstIdImpl: AstId {
  fn default() -> Self;

  fn next_id(&self) -> Self;
}
