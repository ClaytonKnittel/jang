use std::{
  fmt::Debug,
  ops::{Div, Rem},
};

use itertools::Itertools;

use crate::{
  error::{JangError, JangResult},
  interpreter::{bytecode::instruction::JitCompiledFunction, parse_as::ParseAs},
  parser::token::literal::{Literal, NumericLiteral},
};

#[derive(Clone, Debug)]
pub enum Value<'a> {
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
  type Error = JangError;

  fn try_from(value: &Literal) -> JangResult<Self> {
    match value {
      Literal::Numeric(NumericLiteral::Integral(v)) => Ok(Self::Int32(v.parse_as()?)),
      Literal::Numeric(NumericLiteral::Float(v)) => Ok(Self::Float32(v.parse_as()?)),
    }
  }
}

impl<'a> Value<'a> {
  pub fn from_literal(literal: &Literal) -> JangResult<Self> {
    literal.try_into()
  }

  pub fn debug_type_name(&self) -> String {
    match self {
      Self::Int32(_) => "i32".into(),
      Self::Float32(_) => "f32".into(),
      Self::JitCompiledFunctionRef(f) => {
        format!(
          "fn({}){}",
          f.decl()
            .parameters()
            .iter()
            .map(|p| format!("{}", p.ty()))
            .join(", "),
          f.decl()
            .return_type()
            .map_or_else(|| "".into(), |t| format!("->{}", t))
        )
      }
    }
  }

  fn to_numeric_pair(&self, other: &Self) -> JangResult<NumericValuePair> {
    match (self, other) {
      (Self::Int32(a), Self::Int32(b)) => Ok(NumericValuePair::Int32(*a, *b)),
      (Self::Float32(a), Self::Float32(b)) => Ok(NumericValuePair::Float32(*a, *b)),
      (Self::Int32(_), _) | (Self::Float32(_), _) => Err(JangError::interpret_error(format!(
        "type mismatch: {:?} != {:?}",
        self.debug_type_name(),
        other.debug_type_name()
      ))),
      (Self::JitCompiledFunctionRef(_), _) => Err(JangError::interpret_error(format!(
        "non-numeric value: {}",
        self.debug_type_name()
      ))),
    }
  }

  pub fn multiply(&self, other: &Self) -> JangResult<Self> {
    match self
      .to_numeric_pair(other)
      .map_err(|e| e.prefix_msg("multiply: "))?
    {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a * b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a * b)),
    }
  }

  pub fn add(&self, other: &Self) -> JangResult<Self> {
    match self
      .to_numeric_pair(other)
      .map_err(|e| e.prefix_msg("add: "))?
    {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a + b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a + b)),
    }
  }

  pub fn subtract(&self, other: &Self) -> JangResult<Self> {
    match self
      .to_numeric_pair(other)
      .map_err(|e| e.prefix_msg("subtract: "))?
    {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a - b)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a - b)),
    }
  }

  pub fn divide(&self, divisor: &Self) -> JangResult<Self> {
    match self
      .to_numeric_pair(divisor)
      .map_err(|e| e.prefix_msg("divide: "))?
    {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a.checked_div(b).ok_or_else(|| {
        JangError::interpret_error(format!("division by zero: {:?} / {:?}", self, divisor))
      })?)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a.div(b))),
    }
  }

  pub fn modulo(&self, divisor: &Self) -> JangResult<Self> {
    match self
      .to_numeric_pair(divisor)
      .map_err(|e| e.prefix_msg("modulo: "))?
    {
      NumericValuePair::Int32(a, b) => Ok(Value::Int32(a.checked_rem(b).ok_or_else(|| {
        JangError::interpret_error(format!("modulo by zero: {:?} / {:?}", self, divisor))
      })?)),
      NumericValuePair::Float32(a, b) => Ok(Value::Float32(a.rem(b))),
    }
  }

  pub fn is_truthy(&self) -> JangResult<bool> {
    match self {
      Value::Int32(v) => Ok(*v != 0),
      Value::Float32(v) => Ok(*v != 0.),
      v => Err(JangError::interpret_error(format!(
        "unexpected value in truthy check: {:?}",
        v
      ))),
    }
  }

  pub fn as_jit_function(&self) -> JangResult<&'a JitCompiledFunction<'a>> {
    match self {
      Value::JitCompiledFunctionRef(jit_compiled_function) => Ok(*jit_compiled_function),
      value => Err(JangError::interpret_error(format!(
        "expected value to be a JIT-compiled function: {:?}",
        value
      ))),
    }
  }
}
