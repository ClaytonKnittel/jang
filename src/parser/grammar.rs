use parser_generator::grammar;

use crate::parser::token::{JangToken, ident::Ident, keyword::Keyword};

#[derive(Clone, Debug)]
#[allow(unused)]
struct FunctionDecl {
  name: Ident,
}

grammar!(
  name: JangGrammar;
  enum_terminal: JangToken;

  <root>: FunctionDecl => <function_decl>;
  <function_decl>: FunctionDecl =>
    Keyword(Keyword::Function) Ident(..) {
    FunctionDecl {
      name: #1
    }
  };
);

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use parser_generator::parser::Parser;

  use crate::parser::{
    grammar::{FunctionDecl, JangGrammar},
    token::{JangToken, ident::Ident, keyword::Keyword},
  };

  #[gtest]
  fn parse_single_function() {
    let ast = JangGrammar::parse([
      JangToken::Keyword(Keyword::Function),
      JangToken::Ident(Ident::new("test_function")),
    ])
    .unwrap();

    expect_that!(
      ast,
      pat![FunctionDecl {
        name: &Ident::new("test_function")
      }]
    );
  }
}
