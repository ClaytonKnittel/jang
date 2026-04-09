use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub enum InterpreterError {
  Generic(String),
  Internal(String),
  JitCompile(String),
  Unimplemented(String),
  Value(String),
}

impl InterpreterError {
  pub fn generic_err(message: impl Into<String>) -> Self {
    Self::Generic(message.into())
  }

  pub fn internal_err(message: impl Into<String>) -> Self {
    Self::Internal(message.into())
  }

  pub fn jit_err(message: impl Into<String>) -> Self {
    Self::JitCompile(message.into())
  }

  pub fn unimplemented(message: impl Into<String>) -> Self {
    Self::Unimplemented(message.into())
  }

  pub fn value_err(message: impl Into<String>) -> Self {
    Self::Value(message.into())
  }
}

impl Display for InterpreterError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InterpreterError::Generic(s) => write!(f, "{}", s),
      InterpreterError::Unimplemented(s) => write!(f, "unimplemented: {}", s),
      InterpreterError::JitCompile(s) => write!(f, "in jit compiler: {}", s),
      InterpreterError::Value(s) => write!(f, "value error: {}", s),
      InterpreterError::Internal(s) => {
        write!(f, "internal error (bug): {}", s)
      }
    }
  }
}

pub type InterpreterResult<T = ()> = Result<T, InterpreterError>;

#[cfg(test)]
pub(crate) mod matchers {
  use googletest::prelude::*;

  use crate::interpreter::error::InterpreterError;

  pub fn interpreter_value_error<'a>(
    message_matcher: impl Matcher<&'a String>,
  ) -> impl Matcher<&'a InterpreterError> {
    pat!(InterpreterError::Value(message_matcher))
  }
}
