use std::fmt::Display;

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

impl Keyword {
  pub fn build_from_string(string_val: &str) -> Option<Self> {
    match string_val {
      "fn" => Some(Self::Function),
      _ => None,
    }
  }
}

impl Display for Keyword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Function => "fn",
      }
    )
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JangToken {
  Ident(Ident),
  Keyword(Keyword),
}
