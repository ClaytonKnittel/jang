use std::collections::HashMap;

use crate::{
  interpreter::{
    bytecode::{
      compiler::{compile_to_bytecode, instruction::JitCompiledFunction},
      runtime::machine::{self, JitFunctionContext},
    },
    error::{InterpreterError, InterpreterResult},
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
  fn resolve_ident(&'a self, name: &Ident) -> InterpreterResult<Value<'a>> {
    Ok(Value::JitCompiledFunctionRef(
      self
        .program
        .lookup_function(name)
        .ok_or_else(|| InterpreterError::internal_err(format!("unbound variable: {}", name)))?,
    ))
  }
}

impl<'a> Interpreter<'a> {
  pub fn new(jang_file: &'a JangFile) -> InterpreterResult<Self> {
    let function_decls_by_name: HashMap<Ident, JitCompiledFunction<'a>> = jang_file
      .function_decls()
      .iter()
      .map(|f| Ok((f.name_decl().name().clone(), compile_to_bytecode(f)?)))
      .collect::<InterpreterResult<_>>()?;

    Ok(Interpreter {
      program: Program {
        functions: function_decls_by_name,
      },
    })
  }

  pub fn run_main(&self) -> InterpreterResult<i64> {
    let Some(main_fn) = self.program.functions.get(&Ident::new(MAIN_FN_NAME)) else {
      return Err(InterpreterError::generic_err("main function not found"));
    };

    match machine::evaluate_function(main_fn, Vec::new(), self)? {
      Value::Int64(v) => Ok(v),
      Value::Unit => Err(InterpreterError::value_err("main must return a value")),
      r => Err(InterpreterError::value_err(format!(
        "invalid return value from main: {:?}",
        r
      ))),
    }
  }
}

#[cfg(test)]
mod tests {
  use googletest::{expect_that, gtest, prelude::*};

  use crate::{
    interpreter::{
      error::{InterpreterResult, matchers::interpreter_value_error},
      interpreter::Interpreter,
    },
    parser::grammar::testing::lex_and_parse_jang_file,
  };

  fn interpret_program(text: impl IntoIterator<Item = char>) -> InterpreterResult<i64> {
    let ast = lex_and_parse_jang_file(text).unwrap();
    let interp = Interpreter::new(&ast).unwrap();
    interp.run_main()
  }

  #[gtest]
  fn calls_main() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
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
        fn other() -> i64 {
          ret 2
        }

        fn main() -> i64 {
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
        fn add_one(x: i64) -> i64 {
          ret x + 1
        }

        fn main() -> i64 {
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
        fn main() -> i64 {
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
        fn g() -> i64 {
          ret x
        }
        fn f(x: i64) -> i64 {
          ret g()
        }
        fn main() -> i64 {
          ret f(1)
        }
        "#
        .chars()
      ),
      err(displays_as(contains_substring("unbound variable")))
    );
  }

  #[gtest]
  fn let_bindings() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
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
        fn main() -> i64 {
          ret 1 / 0
        }
        "#
        .chars()
      ),
      err(displays_as(contains_substring("division by zero")))
    );
  }

  #[gtest]
  fn if_body() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          if 1 == 1 {
            ret 1
          }
          ret 2
        }
        "#
        .chars()
      ),
      ok(eq(&1))
    );
  }

  #[gtest]
  fn else_body() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          if 1 != 1 {
            ret 1
          } else {
            ret 2
          }
          ret 3
        }
        "#
        .chars()
      ),
      ok(eq(&2))
    );
  }

  #[gtest]
  fn else_if_body() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          if 1 != 1 {
            ret 1
          } else if 1 == 1 {
            ret 2
          } else {
            ret 3
          }
          ret 4
        }
        "#
        .chars()
      ),
      ok(eq(&2))
    );
  }

  #[gtest]
  fn else_if_else_body() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          if 1 != 1 {
            ret 1
          } else if 1 != 1 {
            ret 2
          } else {
            ret 3
          }
          ret 4
        }
        "#
        .chars()
      ),
      ok(eq(&3))
    );
  }

  #[gtest]
  fn fibonacci() {
    expect_that!(
      interpret_program(
        r#"
        fn fib(n: i64) -> i64 {
          if n < 2 { ret 1 }

          ret fib(n - 1) + fib(n - 2)
        }

        fn main() -> i64 {
          ret fib(9)
        }
        "#
        .chars()
      ),
      ok(eq(&55))
    );
  }

  #[gtest]
  fn deeply_recursive_function() {
    expect_that!(
      interpret_program(
        r#"
        fn rec(n: i64) -> i64 {
          if n == 0 { ret 0 }
          ret rec(n - 1) + 1
        }

        fn main() -> i64 {
          ret rec(500)
        }
        "#
        .chars()
      ),
      ok(eq(&500))
    );
  }

  #[gtest]
  fn errors_if_main_does_not_return_a_value() {
    expect_that!(
      interpret_program(
        r#"
        fn main() {}
        "#
        .chars()
      ),
      err(displays_as(contains_substring("main must return a value")))
    );
  }

  #[gtest]
  fn lexical_scope() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          let x = 1
          {
            let x = 2
            if 0 == 0 { ret x }
          }
          ret x
        }
        "#
        .chars()
      ),
      ok(eq(&2))
    );
  }

  #[gtest]
  fn deeply_nested_lexical_scope() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          let x = 0
          let y = 5
          {
            let x = 1 == 1
            { {
                if x { ret y }
            } }
          }
          ret 0
        }
        "#
        .chars()
      ),
      ok(eq(&5))
    );
  }

  #[gtest]
  fn let_in_inner_scope_does_not_affect_outer() {
    expect_that!(
      interpret_program(
        r#"
        fn main() -> i64 {
          let x = 1
          {
            let x = 0
          }
          ret x
        }
        "#
        .chars()
      ),
      ok(eq(&1))
    );
  }
  #[gtest]
  fn function_call_preserves_argument_order_two_args() {
    expect_that!(
      interpret_program(
        r#"
        fn sub(x: i64, y: i64) -> i64 {
          ret x - y
        }

        fn main() -> i64 {
          ret sub(10, 3)
        }
        "#
        .chars()
      ),
      ok(eq(&7))
    );
  }

  #[gtest]
  fn function_call_preserves_argument_order_three_args() {
    expect_that!(
      interpret_program(
        r#"
        fn combine(a: i64, b: i64, c: i64) -> i64 {
          ret a * 100 + b * 10 + c
        }

        fn main() -> i64 {
          ret combine(1, 2, 3)
        }
        "#
        .chars()
      ),
      ok(eq(&123))
    );
  }

  #[gtest]
  fn nested_calls_preserve_argument_order() {
    expect_that!(
      interpret_program(
        r#"
        fn sub(x: i64, y: i64) -> i64 {
          ret x - y
        }

        fn id(x: i64) -> i64 {
          ret x
        }

        fn main() -> i64 {
          ret sub(id(10), id(3))
        }
        "#
        .chars()
      ),
      ok(eq(&7))
    );
  }

  #[gtest]
  fn arithmetic_value_type_mismatch() {
    expect_that!(
      interpret_program(
        r#"
        fn test(x: i64) -> i64 {
          ret x + main
        }

        fn main() -> i64 {
          ret test(1)
        }
        "#
        .chars()
      ),
      err(interpreter_value_error(contains_substring("add")))
    );
  }

  #[gtest]
  fn reassign_in_if_statement() {
    expect_that!(
      interpret_program(
        r#"
        fn test(x: i64) -> i64 {
          mut ret_val = 3
          if x == 2 {
            ret_val = 100
          }
          ret ret_val
        }

        fn main() -> i64 {
          ret test(1) + test(2)
        }
        "#
        .chars()
      ),
      ok(eq(&103))
    );
  }

  #[gtest]
  fn loop_break() {
    expect_that!(
      interpret_program(
        r#"
        fn test(x: i64) -> i64 {
          mut y = x
          mut acc = 1
          loop {
            if y == 0 {
              break
            } else {
              y = y - 1
            }
            acc = acc * 2
          }
          ret acc
        }

        fn main() -> i64 {
          ret test(5)
        }
        "#
        .chars()
      ),
      ok(eq(&32))
    );
  }

  #[gtest]
  fn double_loop() {
    expect_that!(
      interpret_program(
        r#"
        fn test(width: i64, height: i64) -> i64 {
          if width == 0 || height == 0 {
            ret 0
          }

          mut row = 0
          mut acc = 0
          loop {
            mut col = 0
            loop {
              acc = acc + 1
              col = col + 1
              if col == width {
                break
              }
            }

            row = row + 1
            if row == height {
              break
            }
          }
          ret acc
        }

        fn main() -> i64 {
          ret test(10, 13)
        }
        "#
        .chars()
      ),
      ok(eq(&130))
    );
  }

  #[cfg(test)]
  mod examples {
    use crate::interpreter::interpreter::tests::interpret_program;
    use googletest::prelude::*;

    #[gtest]
    fn project_euler_problem1() {
      expect_that!(
        interpret_program(
          r#"
          fn solve(n: i64, acc: i64) -> i64 {
            if n == 1000 { ret acc }
            if n % 3 == 0 { ret solve(n + 1, acc + n) }
            if n % 5 == 0 { ret solve(n + 1, acc + n) }
            ret solve(n + 1, acc)
          }

          fn main() -> i64 {
            ret solve(0, 0)
          }
          "#
          .chars()
        ),
        ok(eq(&233168))
      );
    }

    #[gtest]
    fn project_euler_problem2() {
      expect_that!(
        interpret_program(
          r#"
          fn solve(t0: i64, t1: i64, acc: i64) -> i64 {
            if t0 > 4000000 { ret acc }
            let t2 = t1 + t0
            if t0 % 2 == 0 { ret solve(t1, t2, acc + t0) }
            ret solve(t1, t2, acc)
          }

          fn main() -> i64 {
            ret solve(0, 1, 0)
          }
          "#
          .chars()
        ),
        ok(eq(&4613732))
      );
    }

    #[gtest]
    fn project_euler_problem3() {
      expect_that!(
        interpret_program(
          r#"
          fn largest_prime_factor(n: i64) -> i64 {
            mut n2 = n
            mut p = 2
            loop {
              if n2 == 1 {
                ret p
              } else if n2 % p == 0 {
                n2 = n2 / p
              } else {
                p = p + 1
              }
            }
          }

          fn main() -> i64 {
            ret largest_prime_factor(600851475143)
          }
          "#
          .chars()
        ),
        ok(eq(&6857))
      );
    }

    #[gtest]
    fn project_euler_problem5() {
      expect_that!(
        interpret_program(
          r#"
          fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
              ret a
            } else if a < b {
              ret gcd(b, a)
            } else {
              let d = a / b
              let c = d * b
              ret gcd(b, a - c)
            }
          }

          fn solve(n: i64) -> i64 {
            mut l = 1
            mut i = 2
            loop {
              l = i / gcd(l, i) * l
              i = i + 1
              if i > n {
                break
              }
            }
            ret l
          }

          fn main() -> i64 {
            ret solve(20)
          }
          "#
          .chars()
        ),
        ok(eq(&232792560))
      );
    }

    #[gtest]
    fn project_euler_problem6() {
      expect_that!(
        interpret_program(
          r#"
          fn sum_of_squares(n: i64) -> i64 {
            mut sum = 0
            mut i = 1
            loop {
              sum = sum + i * i
              if i == n { ret sum }
              i = i + 1
            }
          }

          fn square_of_sum(n: i64) -> i64 {
            let sum = n * (n + 1) / 2
            ret sum * sum
          }

          fn main() -> i64 {
            ret square_of_sum(100) - sum_of_squares(100)
          }
          "#
          .chars()
        ),
        ok(eq(&25164150))
      );
    }
  }
}
