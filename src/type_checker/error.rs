use std::fmt::Display;

use crate::{parser::ast::binary_expression::BinaryOp, type_checker::types::registry::Ty};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeCheckerError<'ctx> {
  /// Type mismatch.
  TypeMismatch {
    expected: Ty<'ctx>,
    actual: Ty<'ctx>,
  },
  /// Type mismatch in a binary operation.
  InvalidOperand {
    op: BinaryOp,
    expected: String,
    actual: Ty<'ctx>,
  },
  /// Call target is not a function type.
  NotCallable { target: Ty<'ctx> },
  /// A call passed the wrong number of arguments.
  ArityMismatch { expected: usize, actual: usize },
}

impl<'ctx> Display for TypeCheckerError<'ctx> {
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

impl<'ctx> std::error::Error for TypeCheckerError<'ctx> {}

pub type TypeCheckerResult<'ctx, T = ()> = Result<T, TypeCheckerError<'ctx>>;

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  pub fn type_mismatch_error<'ctx>(
    expected: impl Matcher<&'ctx Ty<'ctx>>,
    actual: impl Matcher<&'ctx Ty<'ctx>>,
  ) -> impl Matcher<&'ctx TypeCheckerError<'ctx>> {
    pat!(TypeCheckerError::TypeMismatch {
      expected: expected,
      actual: actual,
      ..
    })
  }

  pub fn arity_mismatch_error<'ctx>(
    expected: impl Matcher<&'ctx usize>,
    actual: impl Matcher<&'ctx usize>,
  ) -> impl Matcher<&'ctx TypeCheckerError<'ctx>> {
    pat!(TypeCheckerError::ArityMismatch {
      expected: expected,
      actual: actual,
    })
  }

  pub fn not_callable_error<'ctx>(
    target: impl Matcher<&'ctx Ty<'ctx>>,
  ) -> impl Matcher<&'ctx TypeCheckerError<'ctx>> {
    pat!(TypeCheckerError::NotCallable { target: target })
  }

  pub fn invalid_operand<'ctx>(
    expected: impl Matcher<&'ctx String>,
    actual: impl Matcher<&'ctx Ty<'ctx>>,
  ) -> impl Matcher<&'ctx TypeCheckerError<'ctx>> {
    pat!(TypeCheckerError::InvalidOperand {
      expected: expected,
      actual: actual,
      ..
    })
  }
}
