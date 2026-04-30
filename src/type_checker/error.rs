use std::fmt::Display;

use crate::{parser::ast::binary_expression::BinaryOp, type_checker::types::registry::Ty};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeCheckerError<'a> {
  /// Type mismatch.
  TypeMismatch { expected: Ty<'a>, actual: Ty<'a> },
  /// Type mismatch in a binary operation.
  InvalidOperand {
    op: BinaryOp,
    expected: String,
    actual: Ty<'a>,
  },
  /// Call target is not a function type.
  NotCallable { target: Ty<'a> },
  /// A call passed the wrong number of arguments.
  ArityMismatch { expected: usize, actual: usize },
}

impl<'a> Display for TypeCheckerError<'a> {
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

impl<'a> std::error::Error for TypeCheckerError<'a> {}

pub type TypeCheckerResult<'a, T = ()> = Result<T, TypeCheckerError<'a>>;

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  pub fn type_mismatch_error<'a>(
    expected: impl Matcher<&'a Ty<'a>>,
    actual: impl Matcher<&'a Ty<'a>>,
  ) -> impl Matcher<&'a TypeCheckerError<'a>> {
    pat!(TypeCheckerError::TypeMismatch {
      expected: expected,
      actual: actual,
      ..
    })
  }

  pub fn arity_mismatch_error<'a>(
    expected: impl Matcher<&'a usize>,
    actual: impl Matcher<&'a usize>,
  ) -> impl Matcher<&'a TypeCheckerError<'a>> {
    pat!(TypeCheckerError::ArityMismatch {
      expected: expected,
      actual: actual,
    })
  }

  pub fn not_callable_error<'a>(
    target: impl Matcher<&'a Ty<'a>>,
  ) -> impl Matcher<&'a TypeCheckerError<'a>> {
    pat!(TypeCheckerError::NotCallable { target: target })
  }

  pub fn invalid_operand<'a>(
    expected: impl Matcher<&'a String>,
    actual: impl Matcher<&'a Ty<'a>>,
  ) -> impl Matcher<&'a TypeCheckerError<'a>> {
    pat!(TypeCheckerError::InvalidOperand {
      expected: expected,
      actual: actual,
      ..
    })
  }
}
