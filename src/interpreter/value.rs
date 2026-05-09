use std::{
  fmt::Debug,
  ops::{Div, Rem},
};

use cknittel_util::from_variants::FromVariants;

use crate::interpreter::{
  bytecode::compiler::instruction::JitCompiledFunction,
  error::{InterpreterError, InterpreterResult},
};

#[derive(Clone, Debug, FromVariants)]
pub enum PrimitiveValue {
  Bool(bool),
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
}

#[derive(Clone, Debug, FromVariants)]
pub enum Value<'a> {
  Unit,
  Primitive(PrimitiveValue),
  JitCompiledFunctionRef(&'a JitCompiledFunction),
}

/// A pair of two identically-typed numeric values.
#[derive(Debug, Clone)]
enum NumericValuePair {
  Int32(i32, i32),
  Int64(i64, i64),
  Float32(f32, f32),
  Float64(f64, f64),
}

impl<'a> Value<'a> {
  pub fn debug_type_name(&self) -> &'static str {
    match self {
      Self::Unit => "unit",
      Self::Primitive(PrimitiveValue::Bool(_)) => "bool",
      Self::Primitive(PrimitiveValue::Int32(_)) => "i32",
      Self::Primitive(PrimitiveValue::Int64(_)) => "i64",
      Self::Primitive(PrimitiveValue::Float32(_)) => "f32",
      Self::Primitive(PrimitiveValue::Float64(_)) => "f64",
      Self::JitCompiledFunctionRef(_) => "<compiled-bytecode>",
    }
  }

  fn expect_numeric_pair(
    &self,
    other: &Self,
    op: &'static str,
  ) -> InterpreterResult<NumericValuePair> {
    use PrimitiveValue::*;
    use Value::*;
    match (self, other) {
      (Primitive(Int32(a)), Primitive(Int32(b))) => Ok(NumericValuePair::Int32(*a, *b)),
      (Primitive(Int64(a)), Primitive(Int64(b))) => Ok(NumericValuePair::Int64(*a, *b)),
      (Primitive(Float32(a)), Primitive(Float32(b))) => Ok(NumericValuePair::Float32(*a, *b)),
      (Primitive(Float64(a)), Primitive(Float64(b))) => Ok(NumericValuePair::Float64(*a, *b)),
      (lhs, rhs) => Err(InterpreterError::value_err(format!(
        "{op}: expected matching numeric operands, got {} and {}",
        lhs.debug_type_name(),
        rhs.debug_type_name(),
      ))),
    }
  }

  pub fn expect_bool(&self) -> InterpreterResult<bool> {
    match self {
      Value::Primitive(PrimitiveValue::Bool(v)) => Ok(*v),
      _ => Err(InterpreterError::value_err(format!(
        "expected bool, got {} ({:?})",
        self.debug_type_name(),
        self
      ))),
    }
  }

  fn expect_bool_pair(&self, other: &Self, op: &str) -> InterpreterResult<(bool, bool)> {
    match (self, other) {
      (Value::Primitive(PrimitiveValue::Bool(a)), Value::Primitive(PrimitiveValue::Bool(b))) => {
        Ok((*a, *b))
      }
      _ => Err(InterpreterError::value_err(format!(
        "{op}: expected bool, got {} ({self:?})",
        self.debug_type_name(),
      ))),
    }
  }

  pub fn logical_and(&self, other: &Self) -> InterpreterResult<Self> {
    let (a, b) = self.expect_bool_pair(other, "logical and")?;
    Ok(Value::Primitive(PrimitiveValue::Bool(a && b)))
  }

  pub fn logical_or(&self, other: &Self) -> InterpreterResult<Self> {
    let (a, b) = self.expect_bool_pair(other, "logical or")?;
    Ok(Value::Primitive(PrimitiveValue::Bool(a || b)))
  }

  pub fn logical_not(&self) -> InterpreterResult<Self> {
    Ok(Value::Primitive(PrimitiveValue::Bool(!self.expect_bool()?)))
  }

  pub fn multiply(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "multiply")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Int32(a * b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Int64(a * b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Float32(a * b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Float64(a * b))),
    }
  }

  pub fn add(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "add")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Int32(a + b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Int64(a + b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Float32(a + b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Float64(a + b))),
    }
  }

  pub fn subtract(&self, other: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(other, "subtract")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Int32(a - b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Int64(a - b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Float32(a - b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Float64(a - b))),
    }
  }

  pub fn divide(&self, divisor: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(divisor, "divide")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Int32(
        a.checked_div(b).ok_or_else(|| {
          InterpreterError::value_err(format!("division by zero: {:?} / {:?}", self, divisor))
        })?,
      ))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Int64(
        a.checked_div(b).ok_or_else(|| {
          InterpreterError::value_err(format!("division by zero: {:?} / {:?}", self, divisor))
        })?,
      ))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Float32(a.div(b)))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Float64(a.div(b)))),
    }
  }

  pub fn modulo(&self, divisor: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(divisor, "modulo")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Int32(
        a.checked_rem(b).ok_or_else(|| {
          InterpreterError::value_err(format!("modulo by zero: {:?} % {:?}", self, divisor))
        })?,
      ))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Int64(
        a.checked_rem(b).ok_or_else(|| {
          InterpreterError::value_err(format!("modulo by zero: {:?} % {:?}", self, divisor))
        })?,
      ))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Float32(a.rem(b)))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Float64(a.rem(b)))),
    }
  }

  pub fn less_than(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "less than")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a < b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a < b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a < b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a < b))),
    }
  }

  pub fn less_than_equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "less than equal")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a <= b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a <= b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a <= b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a <= b))),
    }
  }

  pub fn greater_than(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "greater than")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a > b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a > b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a > b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a > b))),
    }
  }

  pub fn greater_than_equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    match self.expect_numeric_pair(rhs, "greater than equal")? {
      NumericValuePair::Int32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a >= b))),
      NumericValuePair::Int64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a >= b))),
      NumericValuePair::Float32(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a >= b))),
      NumericValuePair::Float64(a, b) => Ok(Value::Primitive(PrimitiveValue::Bool(a >= b))),
    }
  }

  pub fn equal(&self, rhs: &Self) -> InterpreterResult<Self> {
    use PrimitiveValue::*;
    Ok(Value::Primitive(PrimitiveValue::Bool(match (self, rhs) {
      (Self::Primitive(Bool(a)), Value::Primitive(Bool(b))) => a == b,
      (Self::Primitive(Int32(a)), Value::Primitive(Int32(b))) => a == b,
      (Self::Primitive(Int64(a)), Value::Primitive(Int64(b))) => a == b,
      _ => Err(InterpreterError::value_err(format!(
        "operands not supported for equality, got {} and {}",
        self.debug_type_name(),
        rhs.debug_type_name()
      )))?,
    })))
  }

  pub fn as_jit_function(&self) -> InterpreterResult<&'a JitCompiledFunction> {
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
  use crate::interpreter::value::{PrimitiveValue, Value};
  use googletest::prelude::*;

  pub fn i32_value_primitive<'a>(
    matcher: impl Matcher<&'a i32>,
  ) -> impl Matcher<&'a PrimitiveValue> {
    pat!(PrimitiveValue::Int32(matcher))
  }

  pub fn i64_value_primitive<'a>(
    matcher: impl Matcher<&'a i64>,
  ) -> impl Matcher<&'a PrimitiveValue> {
    pat!(PrimitiveValue::Int64(matcher))
  }

  pub fn i64_value<'a>(matcher: impl Matcher<&'a i64>) -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Primitive(pat!(PrimitiveValue::Int64(matcher))))
  }

  pub fn unit_value<'a>() -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Unit)
  }

  pub fn bool_value<'a>(value: &'a bool) -> impl Matcher<&'a Value<'a>> {
    pat!(Value::Primitive(pat!(PrimitiveValue::Bool(eq(value)))))
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
      Value::Primitive(1.into()).add(&Value::Primitive(2.0.into())),
      err(interpreter_value_error(contains_substring("add:")))
    );
  }

  #[gtest]
  fn arithmetic_op_errors_for_non_numeric_operand() {
    expect_that!(
      Value::Primitive(1.into()).add(&Value::Primitive(true.into())),
      err(interpreter_value_error(contains_substring("add:")))
    );
  }

  #[gtest]
  fn divide_by_zero_errors_for_int32() {
    expect_that!(
      Value::Primitive(1i32.into()).divide(&Value::Primitive(0i32.into())),
      err(interpreter_value_error(contains_substring(
        "division by zero"
      )))
    );
  }

  #[gtest]
  fn less_than() {
    expect_that!(
      Value::Primitive(1i64.into()).less_than(&Value::Primitive(2i64.into())),
      ok(bool_value(&true))
    );
    expect_that!(
      Value::Primitive(1i64.into()).less_than(&Value::Primitive(1i64.into())),
      ok(bool_value(&false))
    );
    expect_that!(
      Value::Primitive(2i64.into()).less_than(&Value::Primitive(1i64.into())),
      ok(bool_value(&false))
    );
    expect_that!(
      Value::Primitive(1f32.into()).less_than(&Value::Primitive(2f32.into())),
      ok(bool_value(&true))
    );
  }

  #[gtest]
  fn equality() {
    expect_that!(
      Value::Primitive(true.into()).equal(&Value::Primitive(true.into())),
      ok(bool_value(&true))
    );
    expect_that!(
      Value::Primitive(7i64.into()).equal(&Value::Primitive(8i64.into())),
      ok(bool_value(&false))
    );
  }

  #[gtest]
  fn equality_with_unsupported_types() {
    expect_that!(
      Value::Primitive(1.0f32.into()).equal(&Value::Primitive(1.0f32.into())),
      err(interpreter_value_error(contains_substring("equality")))
    );

    expect_that!(
      Value::Primitive(true.into()).equal(&Value::Primitive(1.into())),
      err(interpreter_value_error(contains_substring("equality")))
    );
  }

  #[gtest]
  fn expect_bool_rejects_non_bool() {
    expect_that!(
      Value::Primitive(1i64.into()).expect_bool(),
      err(interpreter_value_error(contains_substring(
        "expected bool, got i64"
      )))
    );
  }

  #[gtest]
  fn logical_and() {
    expect_that!(
      Value::Primitive(true.into()).logical_and(&Value::Primitive(false.into())),
      ok(bool_value(&false))
    );
  }

  #[gtest]
  fn logical_or() {
    expect_that!(
      Value::Primitive(true.into()).logical_or(&Value::Primitive(false.into())),
      ok(bool_value(&true))
    );
  }

  #[gtest]
  fn logical_and_with_invalid_type() {
    expect_that!(
      Value::Primitive(123i64.into()).logical_and(&Value::Primitive(false.into())),
      err(interpreter_value_error(contains_substring("logical and")))
    );
  }
}
