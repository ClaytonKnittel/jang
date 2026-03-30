use parser_generator::grammar;

use crate::parser::{
  ast::{
    binary_expression::{BinaryExpression, BinaryOp},
    expression::Expression,
    function_decl::{FunctionDecl, FunctionParameter},
    statement::{LetStatement, RetStatement, Statement},
    type_expr::Type,
  },
  token::{
    JangToken,
    ident::Ident,
    keyword::Keyword,
    operator::{Op, Operator, Spacing},
  },
};

grammar!(
  name: JangGrammar;
  enum_terminal: JangToken;

  <root>: FunctionDecl => <function_decl>;
  <function_decl>: FunctionDecl =>
      Keyword(Keyword::Function)
      <ident>
      <open_paren>
      <parameter_list>
      <close_paren>
      <right_arrow>
      <type>
      <open_bracket>
      <statement_list>
      <close_bracket>
  {
    FunctionDecl::new(#ident, #parameter_list, #type, #statement_list)
  };

  <type>: Type => <ident> { Type(#ident) };

  <statement_list>: Vec<Statement> => ! { Vec::new() };
  <statement_list>: Vec<Statement> => <non_returning_statement_list> <statement> {
    #non_returning_statement_list.push(#statement);
    #non_returning_statement_list
  };

  <non_returning_statement_list>: Vec<Statement> => ! { Vec::new() };
  <non_returning_statement_list>: Vec<Statement> => <non_returning_statement_list> <non_return_statement> {
    #non_returning_statement_list.push(#non_return_statement);
    #non_returning_statement_list
  };

  <statement>: Statement => <non_return_statement>;
  <statement>: Statement => <return_statement>;

  <non_return_statement>: Statement => <let_binding>;
  <let_binding>: Statement => Keyword(Keyword::Let) <ident> <eq> <expr> {
    Statement::Let(LetStatement::new(#ident, #expr))
  };

  <return_statement>: Statement => Keyword(Keyword::Ret) <expr> {
    Statement::Ret(RetStatement::new(#expr))
  };

  <expr>: Expression => <add_expr>;

  <add_expr>: Expression => <add_expr> <plus> <mul_expr> {
    Expression::BinaryExpression(BinaryExpression::new(
      #add_expr, #mul_expr, BinaryOp::Add
    ))
  };
  <add_expr>: Expression => <add_expr> <minus> <mul_expr> {
    Expression::BinaryExpression(BinaryExpression::new(
      #add_expr, #mul_expr, BinaryOp::Sub
    ))
  };
  <add_expr>: Expression => <mul_expr>;

  <mul_expr>: Expression => <mul_expr> <mul> <leaf_expr> {
    Expression::BinaryExpression(BinaryExpression::new(
      #mul_expr, #leaf_expr, BinaryOp::Mul
    ))
  };
  <mul_expr>: Expression => <mul_expr> <div> <leaf_expr> {
    Expression::BinaryExpression(BinaryExpression::new(
      #mul_expr, #leaf_expr, BinaryOp::Div
    ))
  };
  <mul_expr>: Expression => <mul_expr> <modulo> <leaf_expr> {
    Expression::BinaryExpression(BinaryExpression::new(
      #mul_expr, #leaf_expr, BinaryOp::Mod
    ))
  };
  <mul_expr>: Expression => <leaf_expr>;

  <leaf_expr>: Expression => <open_paren> <expr> <close_paren> { #expr };
  <leaf_expr>: Expression => Literal(..) {
    Expression::Literal(#0)
  };
  <leaf_expr>: Expression => Ident(..) {
    Expression::Ident(#0)
  };

  <parameter_list>: Vec<FunctionParameter> => ! { vec![] };
  <parameter_list>: Vec<FunctionParameter> => <non_empty_parameter_list>;
  <non_empty_parameter_list>: Vec<FunctionParameter> => <non_empty_parameter_list> <comma> <ident> <colon> <type> {
    #non_empty_parameter_list.push(FunctionParameter::new(#ident, #type));
    #non_empty_parameter_list
  };
  <non_empty_parameter_list>: Vec<FunctionParameter> => <ident> <colon> <type> {
    vec![
      FunctionParameter::new(#ident, #type)
    ]
  };

  <eq> => Operator(Operator { op: Op::Equal, spacing: Spacing::Alone });
  <eq> => Operator(Operator { op: Op::Equal, spacing: Spacing::Joint });
  <open_paren> => Operator(Operator { op: Op::OpenParen, spacing: Spacing::Alone });
  <open_paren> => Operator(Operator { op: Op::OpenParen, spacing: Spacing::Joint });

  <close_paren> => Operator(Operator { op: Op::CloseParen, spacing: Spacing::Alone });
  <close_paren> => Operator(Operator { op: Op::CloseParen, spacing: Spacing::Joint });

  <open_bracket> => Operator(Operator { op: Op::OpenBracket, spacing: Spacing::Alone });
  <open_bracket> => Operator(Operator { op: Op::OpenBracket, spacing: Spacing::Joint });

  <close_bracket> => Operator(Operator { op: Op::CloseBracket, spacing: Spacing::Alone });
  <close_bracket> => Operator(Operator { op: Op::CloseBracket, spacing: Spacing::Joint });

  <plus> => Operator(Operator { op: Op::Plus, spacing: Spacing::Alone });
  <plus> => Operator(Operator { op: Op::Plus, spacing: Spacing::Joint });

  <minus> => Operator(Operator { op: Op::Dash, spacing: Spacing::Alone });
  <minus> => Operator(Operator { op: Op::Dash, spacing: Spacing::Joint });

  <mul> => Operator(Operator { op: Op::Star, spacing: Spacing::Alone });
  <mul> => Operator(Operator { op: Op::Star, spacing: Spacing::Joint });

  <div> => Operator(Operator { op: Op::Slash, spacing: Spacing::Alone });
  <div> => Operator(Operator { op: Op::Slash, spacing: Spacing::Joint });

  <modulo> => Operator(Operator { op: Op::Percent, spacing: Spacing::Alone });
  <modulo> => Operator(Operator { op: Op::Percent, spacing: Spacing::Joint });

  <colon> => Operator(Operator { op: Op::Colon, spacing: Spacing::Alone });
  <comma> => Operator(Operator { op: Op::Comma, spacing: Spacing::Alone });

  <right_arrow> =>
      Operator(Operator { op: Op::Dash, spacing: Spacing::Joint })
      Operator(Operator { op: Op::GreaterThan, spacing: Spacing::Alone });

  <ident>: Ident => Ident(..);
);

#[cfg(test)]
mod tests {

  use googletest::prelude::*;
  use parser_generator::parser::Parser;

  use crate::parser::{
    ast::{
      binary_expression::{BinaryOp, matchers::binary_expression},
      expression::matchers::{ident_expression, literal_expression},
      function_decl::matchers::{
        fn_body_matches, fn_name_matches, fn_parameter_name_matches, fn_parameter_type_matches,
        fn_parameters_match, fn_return_type_matches,
      },
      statement::matchers::{let_statement, ret_statement},
      type_expr::matchers::type_expr_name_matches,
    },
    grammar::JangGrammar,
    lexer::lex_stream,
    token::{ident::matchers::ident, literal::matchers::integral},
  };

  #[gtest]
  fn parse_minimal_function() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          ret 123
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(ast, fn_name_matches(ident("function_name")));
    expect_that!(
      ast,
      fn_return_type_matches(type_expr_name_matches(ident("i32")))
    );
    expect_that!(
      ast,
      fn_body_matches(elements_are![ret_statement(literal_expression(integral(
        "123"
      )))])
    );
    expect_that!(ast, fn_parameters_match(is_empty()));
  }

  #[gtest]
  fn reject_two_returns() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          ret 123
          ret 456
        }
        "#
      .chars(),
    ));

    expect_that!(ast, err(displays_as(contains_substring("Failed to parse"))));
  }

  #[gtest]
  fn ret_ident() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          ret x
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![ret_statement(ident_expression(ident("x")))])
    );
  }

  #[gtest]
  fn let_binding() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = 123
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        literal_expression(integral("123"))
      )])
    );
  }

  #[gtest]
  fn lets_followed_by_ret() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = 123
          let y = x
          ret 789
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![
        let_statement(ident("x"), literal_expression(integral("123"))),
        let_statement(ident("y"), ident_expression(ident("x"))),
        ret_statement(literal_expression(integral("789")))
      ])
    );
  }

  #[gtest]
  fn reject_let_after_ret() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          ret 123
          let x = 456
        }
        "#
      .chars(),
    ));

    expect_that!(ast, err(displays_as(contains_substring("Failed to parse"))));
  }

  #[gtest]
  fn parse_unary_function() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn func(a: i32) -> i32 {
          ret 123
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_parameters_match(elements_are![all!(
        fn_parameter_name_matches(ident("a")),
        fn_parameter_type_matches(type_expr_name_matches(ident("i32")))
      )])
    );
  }

  #[gtest]
  fn parse_multi_param_function() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn func(a: i32, b: i32) -> i32 {
          ret 123
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_parameters_match(elements_are![
        all!(fn_parameter_name_matches(ident("a"))),
        all!(fn_parameter_name_matches(ident("b")))
      ])
    );
  }

  #[gtest]
  fn add_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = y + 3
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          ident_expression(ident("y")),
          &BinaryOp::Add,
          literal_expression(integral("3"))
        )
      )])
    );
  }

  #[gtest]
  fn sub_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = 5 - a
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          literal_expression(integral("5")),
          &BinaryOp::Sub,
          ident_expression(ident("a")),
        )
      )])
    );
  }

  #[gtest]
  fn mul_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = 2 * a
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          literal_expression(integral("2")),
          &BinaryOp::Mul,
          ident_expression(ident("a")),
        )
      )])
    );
  }

  #[gtest]
  fn div_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = a / b
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          ident_expression(ident("a")),
          &BinaryOp::Div,
          ident_expression(ident("b")),
        )
      )])
    );
  }

  #[gtest]
  fn mod_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = a % 10
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          ident_expression(ident("a")),
          &BinaryOp::Mod,
          literal_expression(integral("10"))
        )
      )])
    );
  }

  #[gtest]
  fn add_left_associativity() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = a + b + c
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          binary_expression(
            ident_expression(ident("a")),
            &BinaryOp::Add,
            ident_expression(ident("b"))
          ),
          &BinaryOp::Add,
          ident_expression(ident("c"))
        )
      )])
    );
  }

  #[gtest]
  fn mul_left_associativity() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = a * b * c
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          binary_expression(
            ident_expression(ident("a")),
            &BinaryOp::Mul,
            ident_expression(ident("b"))
          ),
          &BinaryOp::Mul,
          ident_expression(ident("c"))
        )
      )])
    );
  }

  #[gtest]
  fn add_sub_equal_precedence() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let add_sub = a + b - c
          let sub_add = a - b + c
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![
        let_statement(
          ident("add_sub"),
          binary_expression(
            binary_expression(
              ident_expression(ident("a")),
              &BinaryOp::Add,
              ident_expression(ident("b"))
            ),
            &BinaryOp::Sub,
            ident_expression(ident("c"))
          )
        ),
        let_statement(
          ident("sub_add"),
          binary_expression(
            binary_expression(
              ident_expression(ident("a")),
              &BinaryOp::Sub,
              ident_expression(ident("b"))
            ),
            &BinaryOp::Add,
            ident_expression(ident("c"))
          )
        )
      ])
    );
  }

  #[gtest]
  fn mul_div_mod_equal_precedence() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let mdm = a * b / c % d
          let dmm = a / b % c * d
          let mmd = a % b * c / d
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![
        let_statement(
          ident("mdm"),
          binary_expression(
            binary_expression(
              binary_expression(
                ident_expression(ident("a")),
                &BinaryOp::Mul,
                ident_expression(ident("b"))
              ),
              &BinaryOp::Div,
              ident_expression(ident("c"))
            ),
            &BinaryOp::Mod,
            ident_expression(ident("d"))
          )
        ),
        let_statement(
          ident("dmm"),
          binary_expression(
            binary_expression(
              binary_expression(
                ident_expression(ident("a")),
                &BinaryOp::Div,
                ident_expression(ident("b"))
              ),
              &BinaryOp::Mod,
              ident_expression(ident("c"))
            ),
            &BinaryOp::Mul,
            ident_expression(ident("d"))
          )
        ),
        let_statement(
          ident("mmd"),
          binary_expression(
            binary_expression(
              binary_expression(
                ident_expression(ident("a")),
                &BinaryOp::Mod,
                ident_expression(ident("b"))
              ),
              &BinaryOp::Mul,
              ident_expression(ident("c"))
            ),
            &BinaryOp::Div,
            ident_expression(ident("d"))
          )
        )
      ])
    );
  }

  #[gtest]
  fn parenthesis_expression() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = (y + z) - (w * 3)
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          binary_expression(
            ident_expression(ident("y")),
            &BinaryOp::Add,
            ident_expression(ident("z"))
          ),
          &BinaryOp::Sub,
          binary_expression(
            ident_expression(ident("w")),
            &BinaryOp::Mul,
            literal_expression(integral("3"))
          )
        )
      )])
    );
  }

  #[gtest]
  fn many_parenthesis() {
    let ast = JangGrammar::parse_fallible(lex_stream(
      r#"
        fn function_name() -> i32 {
          let x = (((((((((y))))) + (((((((z)))))))))))
        }
        "#
      .chars(),
    ))
    .unwrap();

    expect_that!(
      ast,
      fn_body_matches(elements_are![let_statement(
        ident("x"),
        binary_expression(
          ident_expression(ident("y")),
          &BinaryOp::Add,
          ident_expression(ident("z"))
        )
      )])
    );
  }
}
