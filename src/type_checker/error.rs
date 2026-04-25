use std::fmt::Display;

use crate::{
  parser::ast::binary_expression::BinaryOp, type_checker::type_inference_table::InferredType,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeCheckerError {
  /// Type mismatch.
  TypeMismatch {
    expected: InferredType,
    actual: InferredType,
  },
  /// Type mismatch.
  InvalidOperand {
    op: BinaryOp,
    expected: String,
    actual: InferredType,
  },
  /// Call target is not a function type.
  NotCallable { target: InferredType },
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
  use crate::type_checker::{
    type_inference_table::matchers::concrete, types::concrete::ConcreteType,
  };

  use super::*;
  use googletest::prelude::*;

  pub fn type_mismatch_error<'a>(
    expected: impl Matcher<&'a InferredType>,
    actual: impl Matcher<&'a InferredType>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::TypeMismatch {
      expected: expected,
      actual: actual,
      ..
    })
  }

  // Type mismatch between two concrete types.
  pub fn concrete_type_mismatch_error<'a>(
    expected: impl Matcher<&'a ConcreteType>,
    actual: impl Matcher<&'a ConcreteType>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::TypeMismatch {
      expected: concrete(expected),
      actual: concrete(actual),
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
    target: impl Matcher<&'a ConcreteType>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::NotCallable {
      target: pat!(InferredType::Concrete(target))
    })
  }

  pub fn invalid_operand<'a>(
    expected: impl Matcher<&'a String>,
    actual: impl Matcher<&'a ConcreteType>,
  ) -> impl Matcher<&'a TypeCheckerError> {
    pat!(TypeCheckerError::InvalidOperand {
      expected: expected,
      actual: pat!(InferredType::Concrete(actual)),
      ..
    })
  }
}
