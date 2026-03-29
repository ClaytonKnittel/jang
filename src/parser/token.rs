#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
  pub fn new(name: impl Into<String>) -> Self {
    Self(name.into())
  }
}

#[derive(Clone)]
pub enum Keyword {
  Function,
}

#[derive(Clone)]
pub enum JangToken {
  Ident(Ident),
  Keyword(Keyword),
}

impl JangToken {
  pub fn into_ident(self) -> Ident {
    match self {
      Self::Ident(ident) => ident,
      _ => panic!("Not an ident!"),
    }
  }
}
