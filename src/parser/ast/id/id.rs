pub trait AstId: Copy {}

pub(super) trait AstIdImpl: AstId {
  fn default() -> Self;

  fn next_id(&self) -> Self;

  /// Returns the underlying integer of the ID.
  /// Relies on IDs being allocated sequentially.
  fn as_index(&self) -> usize;

  /// Constructs an ID from an index.
  /// Relies on IDs being allocated sequentially.
  fn from_index(index: usize) -> Self;
}
