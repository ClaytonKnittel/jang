use std::{
  convert::Infallible,
  error::Error,
  fmt::{Debug, Display},
};

use parser_generator::error::ParserError;

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
  GrammarError(ParserError<Infallible>),
  JitCompilerError(),
}

impl JangError {
  pub fn parse_error(message: impl Into<String>, source_location: SourceLocation) -> Self {
    Self::ParseError(ParseError::new(message, source_location))
  }
}

impl From<ParserError<JangError>> for JangError {
  fn from(value: ParserError<JangError>) -> Self {
    match value {
      ParserError::InputStreamError(err) => err,
      ParserError::ParseError { message } => {
        JangError::GrammarError(ParserError::ParseError { message })
      }
      ParserError::ForeignError { message } => {
        JangError::GrammarError(ParserError::ForeignError { message })
      }
      #[cfg(debug_assertions)]
      ParserError::OverlappingTokenMatchers { token } => {
        JangError::GrammarError(ParserError::OverlappingTokenMatchers { token })
      }
    }
  }
}

impl Error for JangError {}

impl Display for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParseError(err) => write!(f, "{err}"),
      Self::GrammarError(err) => write!(f, "Grammar error: {err}"),
      Self::JitCompilerError() => write!(f, "JIT Compiler error"),
    }
  }
}

impl Debug for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type JangResult<T = ()> = Result<T, JangError>;
