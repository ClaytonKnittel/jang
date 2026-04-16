use std::{error::Error, fs::File, io::BufReader};

use googletest::prelude::*;
use jang::{
  error::{JangError, JangResult},
  interpreter::Interpreter,
  parser::lex_and_parse_jang_file,
};
use utf8_chars::BufReadCharsExt;

fn run_program<E: Error>(
  text: impl IntoIterator<Item = std::result::Result<char, E>>,
) -> JangResult<i64> {
  let ast = lex_and_parse_jang_file(text)?;
  let interp = Interpreter::new(&ast)?;
  interp.run_main().map_err(JangError::Interpret)
}

macro_rules! test_return_val {
  (
    $($test_name:ident($file_path:expr => $ret_val:expr))+
  ) => {
    $(#[gtest]
    fn $test_name() -> Result<()> {
      let file = File::open($file_path)?;
      let result = run_program(BufReader::new(file).chars());
      verify_that!(result, ok(eq(&$ret_val)))
    })+
  };
}

test_return_val!(
  test_euler_problem1("./jang-examples/euler_problem1.jang" => 233168)
  test_euler_problem3("./jang-examples/euler_problem3.jang" => 6857)
  test_euler_problem6("./jang-examples/euler_problem6.jang" => 25164150)
  test_euler_problem7("./jang-examples/euler_problem7.jang" => 104743)
);
