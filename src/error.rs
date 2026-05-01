use std::{
  convert::Infallible,
  error::Error,
  fmt::{Debug, Display},
};

use cknittel_util::builder::error::BuilderError;
use parser_generator::{ParserUserError, error::ParserError};

use crate::{
  interpreter::error::InterpreterError,
  parser::token::{JangToken, ident::Ident},
  source_location::SourceSpan,
};

#[derive(Clone)]
pub enum ParseErrorKind {
  UnexpectedSymbol { symbol: char },
  DuplicateIdent { ident: String },
}

impl Display for ParseErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::UnexpectedSymbol { symbol } => {
        write!(f, "Unexpected symbol: '{symbol}'")
      }
      Self::DuplicateIdent { ident } => {
        write!(f, "Duplicate identifier: \"{ident}\"")
      }
    }
  }
}

impl Debug for ParseErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

#[derive(Clone)]
pub struct ParseError {
  kind: ParseErrorKind,
  span: SourceSpan,
}

impl ParseError {
  pub fn kind(&self) -> &ParseErrorKind {
    &self.kind
  }

  pub fn span(&self) -> &SourceSpan {
    &self.span
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} (span: {})", self.kind, self.span)
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

#[derive(Clone, ParserUserError)]
pub enum JangError {
  Parse(ParseError),
  Grammar(ParserError<JangToken, Infallible>),
  Builder(BuilderError),
  Interpret(InterpreterError),
  ForeignError(String),
}

impl JangError {
  pub fn unexpected_symbol(symbol: char, span: SourceSpan) -> Self {
    Self::Parse(ParseError {
      kind: ParseErrorKind::UnexpectedSymbol { symbol },
      span,
    })
  }

  pub fn duplicate_ident(ident: &Ident) -> Self {
    Self::Parse(ParseError {
      kind: ParseErrorKind::DuplicateIdent {
        ident: ident.name().to_owned(),
      },
      span: ident.span().clone(),
    })
  }
}

impl From<ParserError<JangToken, JangError>> for JangError {
  fn from(value: ParserError<JangToken, JangError>) -> Self {
    match value {
      ParserError::UserError(err) => err,
      ParserError::ParseError { next_token } => {
        JangError::Grammar(ParserError::ParseError { next_token })
      }
      #[cfg(debug_assertions)]
      ParserError::OverlappingTokenMatchers { token } => {
        JangError::Grammar(ParserError::OverlappingTokenMatchers { token })
      }
    }
  }
}

impl From<BuilderError> for JangError {
  fn from(value: BuilderError) -> Self {
    JangError::Builder(value)
  }
}

impl From<InterpreterError> for JangError {
  fn from(err: InterpreterError) -> Self {
    JangError::Interpret(err)
  }
}

impl Error for JangError {}

impl Display for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Parse(err) => write!(f, "{err}"),
      Self::Grammar(err) => write!(f, "Grammar error: {err}"),
      Self::Builder(err) => write!(f, "Builder error: {err}"),
      Self::Interpret(err) => write!(f, "Interpret error: {err}"),
      Self::ForeignError(message) => write!(f, "{message}"),
    }
  }
}

impl Debug for JangError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type JangResult<T = ()> = Result<T, JangError>;

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  pub fn grammar_err(next_token: &Option<JangToken>) -> impl Matcher<&JangError> {
    pat!(JangError::Grammar(pat!(ParserError::ParseError {
      next_token: eq(next_token)
    })))
  }

  pub fn unexpected_symbol(symbol: &char) -> impl Matcher<&JangError> {
    pat!(JangError::Parse(property!(
      &ParseError.kind(),
      pat!(ParseErrorKind::UnexpectedSymbol { symbol: eq(symbol) })
    )))
  }

  pub fn err_span(span: &SourceSpan) -> impl Matcher<&JangError> {
    pat!(JangError::Parse(property!(&ParseError.span(), eq(span))))
  }
}
