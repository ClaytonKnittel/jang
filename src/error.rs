use std::{
  error::Error,
  fmt::{Debug, Display},
};

use crate::source_location::SourceLocation;

#[derive(Clone)]
pub struct ParseError {
  message: String,
  source_location: SourceLocation,
}

impl ParseError {
  fn new(message: impl Into<String>, source_location: SourceLocation) -> Self {
    Self {
      message: message.into(),
      source_location,
    }
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Parse error: {} (pos: {})",
      self.message,
      self.source_location.pos()
    )
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

#[derive(Clone)]
pub enum JangError {
  ParseError(ParseError),
}

impl JangError {
  pub fn parse_error(message: impl Into<String>, source_location: SourceLocation) -> Self {
    Self::ParseError(ParseError::new(message, source_location))
  }
}

impl Error for JangError {}

impl Display for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError(parse_error) => write!(f, "{parse_error}"),
    }
  }
}

impl Debug for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type JangResult<T = ()> = Result<T, JangError>;
