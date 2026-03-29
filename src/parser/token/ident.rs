#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }

  pub fn name(&self) -> &str {
    &self.0
  }
}


