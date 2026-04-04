use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct TypeDecl {}

impl Display for TypeDecl {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
