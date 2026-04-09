use std::{
  fmt::{Debug, Display},
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
  Int32(i32),
  Float32(f32),
  JitCompiledFunctionRef(&'a JitCompiledFunction<'a>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
  Unit,
  Int32,
  Float32,
  Function,
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

  pub fn ty(&self) -> ValueKind {
    match self {
      Self::Unit => ValueKind::Unit,
      Self::Int32(_) => ValueKind::Int32,
      Self::Float32(_) => ValueKind::Float32,
      Self::JitCompiledFunctionRef(_) => ValueKind::Function,
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
        lhs.ty(),
        rhs.ty(),
      ))),
    }
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

  pub fn is_truthy(&self) -> InterpreterResult<bool> {
    match self {
      Value::Int32(v) => Ok(*v != 0),
      value => Err(InterpreterError::value_err(format!(
        "unexpected value in truthy check: {:?}",
        value
      ))),
    }
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

impl Display for ValueKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ValueKind::Unit => "unit",
        ValueKind::Int32 => "i32",
        ValueKind::Float32 => "f32",
        ValueKind::Function => "<compiled-bytecode>",
      }
    )
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
}
