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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
  Function,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JangToken {
  Ident(Ident),
  Keyword(Keyword),
}
