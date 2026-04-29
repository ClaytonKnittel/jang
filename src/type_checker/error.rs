use std::fmt::Display;

use crate::parser::ast::binary_expression::BinaryOp;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeCheckerError {
  /// Type mismatch.
  TypeMismatch { expected: String, actual: String },
  /// Type mismatch in a binary operation.
  InvalidOperand {
    op: BinaryOp,
    expected: String,
    actual: String,
  },
  /// Call target is not a function type.
  NotCallable { target: String },
  /// A call passed the wrong number of arguments.
  ArityMismatch { expected: usize, actual: usize },
}

impl Display for TypeCheckerError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TypeMismatch { expected, actual } => {
        write!(f, "expected `{expected}`, but found `{actual}`")
      }
      Self::InvalidOperand {
        op,
        expected,
        actual,
      } => {
        write!(f, "in `{op}` required {expected}, but found `{actual}`")
      }
      Self::NotCallable { target } => write!(f, "cannot call a value of type `{target}`"),
      Self::ArityMismatch { expected, actual } => write!(
        f,
        "wrong number of arguments: expected {expected}, but found {actual}"
      ),
    }
  }
}

impl std::error::Error for TypeCheckerError {}

pub type TypeCheckerResult<T = ()> = Result<T, TypeCheckerError>;

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  pub fn type_mismatch_error<'a>(
    expected: impl Matcher<&'a str>,
    actual: impl Matcher<&'a str>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::TypeMismatch {
      expected: result_of!(&String::as_str, expected),
      actual: result_of!(&String::as_str, actual),
      ..
    })
  }

  pub fn arity_mismatch_error<'a>(
    expected: impl Matcher<&'a usize>,
    actual: impl Matcher<&'a usize>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::ArityMismatch {
      expected: expected,
      actual: actual,
    })
  }

  pub fn not_callable_error<'a>(
    target: impl Matcher<&'a str>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::NotCallable {
      target: result_of!(&String::as_str, target)
    })
  }

  pub fn invalid_operand<'a>(
    expected: impl Matcher<&'a str>,
    actual: impl Matcher<&'a str>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::InvalidOperand {
      expected: result_of!(&String::as_str, expected),
      actual: result_of!(&String::as_str, actual),
      ..
    })
  }
}
