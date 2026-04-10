use std::{
  fmt::Debug,
  ops::{Div, Rem},
};

use crate::{
  interpreter::{
    bytecode::instruction::JitCompiledFunction,
    error::{InterpreterError, InterpreterResult},
    parse_as::ParseAs,
  },
  parser::token::literal::{Literal, NumericLiteral},
};

#[derive(Clone, Debug)]
pub enum Value<'a> {
  Unit,
  Bool(bool),
  Int32(i32),
  Float32(f32),
  JitCompiledFunctionRef(&'a JitCompiledFunction<'a>),
}

/// A pair of two identically-typed numeric values.
#[derive(Debug, Clone)]
enum NumericValuePair {
  Int32(i32, i32),
  Float32(f32, f32),
}

impl<'a> TryFrom<&Literal> for Value<'a> {
  type Error = InterpreterError;

  fn try_from(value: &Literal) -> InterpreterResult<Self> {
    match value {
      Literal::Numeric(NumericLiteral::Integral(v)) => Ok(Self::Int32(v.parse_as()?)),
      Literal::Numeric(NumericLiteral::Float(v)) => Ok(Self::Float32(v.parse_as()?)),
    }
  }
}

impl<'a> Value<'a> {
  pub fn from_literal(literal: &Literal) -> InterpreterResult<Self> {
    literal.try_into()
  }

  pub fn debug_type_name(&self) -> &'static str {
    match self {
      Self::Unit => "unit",
      Self::Bool(_) => "bool",
      Self::Int32(_) => "i32",
      Self::Float32(_) => "f32",
      Self::JitCompiledFunctionRef(_) => "<compiled-bytecode>",
    }
  }

  fn expect_numeric_pair(
    &self,
    other: &Self,
    op: &'static str,
  ) -> InterpreterResult<NumericValuePair> {
    match (self, other) {
      (Value::Int32(a), Value::Int32(b)) => Ok(NumericValuePair::Int32(*a, *b)),
      (Value::Float32(a), Value::Float32(b)) => Ok(NumericValuePair::Float32(*a, *b)),
      (lhs, rhs) => Err(InterpreterError::value_err(format!(
        "{op}: expected matching numeric operands, got {} and {}",
        lhs.debug_type_name(),
        rhs.debug_type_name(),
      ))),
    }
  }

  pub fn expect_bool(&self) -> InterpreterResult<bool> {
    match self {
      Value::Bool(v) => Ok(*v),
      _ => Err(InterpreterError::value_err(format!(
        "expected bool, got {} ({:?})",
        self.debug_type_name(),
        self
      ))),
    }
  }

  fn expect_bool_pair(&self, other: &Self, op: &str) -> InterpreterResult<(bool, bool)> {
    match (self, other) {
      (Value::Bool(a), Value::Bool(b)) => Ok((*a, *b)),
      _ => Err(InterpreterError::value_err(format!(
        "{op}: expected bool, got {} ({self:?})",
        self.debug_type_name(),
      ))),
    }
  }

  pub fn logical_and(&self, other: &Self) -> InterpreterResult<Self> {
    let (a, b) = self.expect_bool_pair(other, "logical and")?;
    Ok(Value::Bool(a && b))
  }

  pub fn logical_or(&self, other: &Self) -> InterpreterResult<Self> {
    let (a, b) = self.expect_bool_pair(other, "logical or")?;
    Ok(Value::Bool(a || b))
  }

  pub fn logical_not(&self) -> InterpreterResult<Self> {
    Ok(Value::Bool(!self.expect_bool()?))
  }

  pub fn multiply(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "multiply")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a * b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a * b)),
    }
  }

  pub fn add(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "add")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a + b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a + b)),
    }
  }

  pub fn subtract(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "subtract")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a - b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a - b)),
    }
  }

  pub fn divide(&self, divisor: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(divisor, "divide")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a.checked_div(b).ok_or_else(|| {
        InterpreterError::value_err(format!("division by zero: {:?} / {:?}", self, divisor))
      })?)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a.div(b))),
    }
  }

  pub fn modulo(&self, divisor: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(divisor, "modulo")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a.checked_rem(b).ok_or_else(|| {
        InterpreterError::value_err(format!("modulo by zero: {:?} / {:?}", self, divisor))
      })?)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a.rem(b))),
    }
  }

  pub fn less_than(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "less than")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Bool(a < b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Bool(a < b)),
    }
  }

  pub fn less_than_equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "less than equal")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Bool(a <= b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Bool(a <= b)),
    }
  }

  pub fn greater_than(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "greater than")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Bool(a > b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Bool(a > b)),
    }
  }

  pub fn greater_than_equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "greater than equal")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Bool(a >= b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Bool(a >= b)),
    }
  }

  pub fn equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    Ok(Value::Bool(match (self, rhs) {
      (Self::Bool(a), Value::Bool(b)) => a == b,
      (Self::Int32(a), Value::Int32(b)) => a == b,
      _ => Err(InterpreterError::value_err(format!(
        "operands not supported for equality, got {} and {}",
        self.debug_type_name(),
        rhs.debug_type_name()
      )))?,
    }))
  }

  pub fn as_jit_function(&self) -> InterpreterResult<&'a JitCompiledFunction<'a>> {
    match self {
      Value::JitCompiledFunctionRef(jit_compiled_function) => Ok(*jit_compiled_function),
      value => Err(InterpreterError::value_err(format!(
        "expected value to be a JIT-compiled function: {:?}",
        value
      ))),
    }
  }
}

#[cfg(test)]
pub mod matchers {
  use crate::interpreter::value::Value;
  use googletest::prelude::*;

  pub fn i32_value<'a>(matcher: impl Matcher<&'a i32>) -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Int32(matcher))
  }

  pub fn unit_value<'a>() -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Unit)
  }

  pub fn bool_value<'a>(value: &'a bool) -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Bool(eq(value)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::interpreter::{error::matchers::interpreter_value_error, value::matchers::bool_value};
  use googletest::prelude::*;

  #[gtest]
  fn arithmetic_op_errors_for_mismatched_types() {
    expect_that!(
      Value::Int32(1).add(&Value::Float32(2.0)),
      err(interpreter_value_error(contains_substring("add:")))
    );
  }

  #[gtest]
  fn arithmetic_op_errors_for_non_numeric_operand() {
    expect_that!(
      Value::Int32(1).add(&Value::Bool(true)),
      err(interpreter_value_error(contains_substring("add:")))
    );
  }

  #[gtest]
  fn divide_by_zero_errors_for_int32() {
    expect_that!(
      Value::Int32(1).divide(&Value::Int32(0)),
      err(interpreter_value_error(contains_substring(
        "division by zero"
      )))
    );
  }

  #[gtest]
  fn less_than() {
    expect_that!(
      Value::Int32(1).less_than(&Value::Int32(2)),
      ok(bool_value(&true))
    );
    expect_that!(
      Value::Int32(1).less_than(&Value::Int32(1)),
      ok(bool_value(&false))
    );
    expect_that!(
      Value::Int32(2).less_than(&Value::Int32(1)),
      ok(bool_value(&false))
    );

    expect_that!(
      Value::Float32(1.).less_than(&Value::Float32(2.)),
      ok(bool_value(&true))
    );
  }

  #[gtest]
  fn equality() {
    expect_that!(
      Value::Bool(true).equal(&Value::Bool(true)),
      ok(bool_value(&true))
    );
    expect_that!(
      Value::Int32(7).equal(&Value::Int32(8)),
      ok(bool_value(&false))
    );
  }

  #[gtest]
  fn equality_with_unsupported_types() {
    expect_that!(
      Value::Float32(1.0).equal(&Value::Float32(1.0)),
      err(interpreter_value_error(contains_substring("equality")))
    );

    expect_that!(
      Value::Bool(true).equal(&Value::Int32(1)),
      err(interpreter_value_error(contains_substring("equality")))
    );
  }

  #[gtest]
  fn expect_bool_rejects_non_bool() {
    expect_that!(
      Value::Int32(1).expect_bool(),
      err(interpreter_value_error(contains_substring(
        "expected bool, got i32"
      )))
    );
  }

  #[gtest]
  fn logical_and() {
    expect_that!(
      Value::Bool(true).logical_and(&Value::Bool(false)),
      ok(bool_value(&false))
    );
  }

  #[gtest]
  fn logical_or() {
    expect_that!(
      Value::Bool(true).logical_or(&Value::Bool(false)),
      ok(bool_value(&true))
    );
  }

  #[gtest]
  fn logical_and_with_invalid_type() {
    expect_that!(
      Value::Int32(123).logical_and(&Value::Bool(false)),
      err(interpreter_value_error(contains_substring("logical and")))
    );
  }
}
