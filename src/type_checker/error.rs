use std::fmt::Display;

use crate::{
  parser::token::ident::Ident,
  type_checker::{inference::TypeClass, types::registry::Ty},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeCheckerError<'ctx> {
  /// Type mismatch.
  TypeMismatch {
    expected: Ty<'ctx>,
    actual: Ty<'ctx>,
  },
  /// Type class mismatch.
  TypeClassMismatch {
    expected: TypeClass,
    actual: Ty<'ctx>,
  },
  /// Call target is not a function type.
  NotCallable { target: Ty<'ctx> },
  /// Bad member access.
  InvalidMemberAccess { target: Ty<'ctx>, member: Ident },
  /// A call passed the wrong number of arguments.
  ArityMismatch { expected: usize, actual: usize },
  /// A variable reference could not be resolved.
  UndefinedVariable,
}

impl<'ctx> Display for TypeCheckerError<'ctx> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TypeMismatch { expected, actual } => {
        write!(f, "expected `{expected}`, but found `{actual}`")
      }
      Self::TypeClassMismatch { expected, actual } => {
        write!(f, "required {expected} type, but found `{actual}`")
      }
      Self::NotCallable { target } => write!(f, "cannot call a value of type `{target}`"),
      Self::InvalidMemberAccess { target, member } => {
        write!(f, "cannot access member \"{member}\" on type `{target}`")
      }
      Self::ArityMismatch { expected, actual } => write!(
        f,
        "wrong number of arguments: expected {expected}, but found {actual}"
      ),
      Self::UndefinedVariable => write!(f, "unbound variable"),
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

  pub fn type_class_mismatch<'ctx>(
    expected: impl Matcher<&'ctx TypeClass>,
    actual: impl Matcher<&'ctx Ty<'ctx>>,
  ) -> impl Matcher<&'ctx TypeCheckerError<'ctx>> {
    pat!(TypeCheckerError::TypeClassMismatch {
      expected: expected,
      actual: actual,
      ..
    })
  }

  pub fn invalid_member_access<'a>(
    target: impl Matcher<&'a Ty<'a>>,
    member: impl Matcher<&'a Ident>,
  ) -> impl Matcher<&'a TypeCheckerError<'a>> {
    pat!(TypeCheckerError::InvalidMemberAccess {
      target: target,
      member: member
    })
  }
}
