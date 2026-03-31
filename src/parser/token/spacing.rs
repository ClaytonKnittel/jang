#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacing {
  /// This token is followed by whitespace.
  Alone,
  /// This token is followed by the next token without any whitespace
  /// separation.
  Joint,
}
