use std::{fmt::Debug, str::FromStr};

use crate::interpreter::error::{InterpreterError, InterpreterResult};

pub trait ParseAs<T> {
  fn parse_as(self) -> InterpreterResult<T>;
}

impl<T, E> ParseAs<T> for &str
where
  E: Debug,
  T: FromStr<Err = E>,
{
  fn parse_as(self) -> InterpreterResult<T> {
    self
      .parse()
      .map_err(|err: <T as FromStr>::Err| InterpreterError::value_err(format!("{:?}", err)))
  }
}
