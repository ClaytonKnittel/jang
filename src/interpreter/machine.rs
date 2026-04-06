use std::collections::HashMap;

use crate::{
  error::{JangError, JangResult},
  interpreter::{
    bytecode::{
      compiler::compile_to_bytecode,
      instruction::JitCompiledFunction,
      machine::{self, JitFunctionContext},
    },
    value::Value,
  },
  parser::{ast::jang_file::JangFile, token::ident::Ident},
};

const MAIN_FN_NAME: &str = "main";

struct Program<'a> {
  functions: HashMap<Ident, JitCompiledFunction<'a>>,
}

impl<'a> Program<'a> {
  pub fn lookup_function(&self, var: &Ident) -> Option<&JitCompiledFunction<'a>> {
    self.functions.get(var)
  }
}

pub struct Interpreter<'a> {
  program: Program<'a>,
}

impl<'a> JitFunctionContext<'a> for Interpreter<'a> {
  fn resolve_ident(&'a self, name: &Ident) -> JangResult<Value<'a>> {
    Ok(Value::JitCompiledFunctionRef(
      self
        .program
        .lookup_function(name)
        .ok_or_else(|| JangError::interpret_error(format!("identifier not found: {:?}", name)))?,
    ))
  }
}

impl<'a> Interpreter<'a> {
  pub fn new(jang_file: &'a JangFile) -> JangResult<Self> {
    let function_decls_by_name: HashMap<Ident, JitCompiledFunction<'a>> = jang_file
      .function_decls()
      .iter()
      .map(|f| Ok((f.name().clone(), compile_to_bytecode(f)?)))
      .collect::<JangResult<_>>()?;

    Ok(Interpreter {
      program: Program {
        functions: function_decls_by_name,
      },
    })
  }

  pub fn run_main(&self) -> JangResult<i32> {
    let Some(main_fn) = self.program.functions.get(&Ident::new(MAIN_FN_NAME)) else {
      todo!();
    };

    match machine::evaluate_function(main_fn, Vec::new(), self)? {
      Value::Int32(v) => Ok(v),
      _ => todo!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use googletest::{expect_that, gtest, prelude::*};

  use crate::{
    error::JangResult, interpreter::machine::Interpreter,
    parser::grammar::testing::lex_and_parse_jang_file,
  };

  fn interpret_program(text: impl IntoIterator<Item = char>) -> JangResult<i32> {
    let ast = lex_and_parse_jang_file(text).unwrap();
    let interp = Interpreter::new(&ast).unwrap();
    interp.run_main()
  }

  #[gtest]
  fn calls_main() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i32 {
          ret 1
        }
        "#
        .chars()
      ),
      ok(eq(&1))
    );
  }

  #[gtest]
  fn fn_call() {
    expect_that!(
      interpret_program(
        r#"
        fn other() -> i32 {
          ret 2
        }

        fn main() -> i32 {
          ret 1 + other()
        }
        "#
        .chars()
      ),
      ok(eq(&3))
    );
  }

  #[gtest]
  fn locals() {
    expect_that!(
      interpret_program(
        r#"
        fn add_one(x: i32) -> i32 {
          ret x + 1
        }

        fn main() -> i32 {
          ret add_one(1) + 2 * add_one(2)
        }
        "#
        .chars()
      ),
      ok(eq(&8))
    );
  }

  #[gtest]
  fn integer_arithmetic() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i32 {
          ret (8 + 9 % 5) / 2 * (7 - 3) + 6 - 5
        }
        "#
        .chars()
      ),
      ok(eq(&25))
    );
  }

  #[gtest]
  fn unbound_variable_throws_error() {
    expect_that!(
      interpret_program(
        r#"
        fn g() -> i32 {
          ret x
        }
        fn f(x: i32) -> i32 {
          ret g()
        }
        fn main() -> i32 {
          ret f(1)
        }
        "#
        .chars()
      ),
      err(anything())
    );
  }

  #[gtest]
  fn let_bindings() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i32 {
          let x = 1
          let y = 2
          ret x + y
        }
        "#
        .chars()
      ),
      ok(eq(&3))
    );
  }

  #[gtest]
  fn divide_by_zero() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i32 {
          ret 1 / 0
        }
        "#
        .chars()
      ),
      err(anything())
    );
  }

  #[gtest]
  fn if_statement() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i32 {
          if 1 {
            ret 1
          }
          ret 0
        }
        "#
        .chars()
      ),
      ok(eq(&1))
    );
  }

  #[gtest]
  fn fibonacci() {
    expect_that!(
      interpret_program(
        r#"
        fn fib(n: i32) -> i32 {
          if n - 1 {} else { ret 1 }
          if n {} else { ret 1 }

          ret fib(n - 1) + fib(n - 2)
        }

        fn main() -> i32 {
          ret fib(9)
        }
        "#
        .chars()
      ),
      ok(eq(&55))
    );
  }
}
