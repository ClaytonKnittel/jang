use jang::{
  parser::{grammar::JangGrammar, lexer::lex_stream},
  write_ast,
};
use parser_generator::parser::Parser;

fn main() {
  let code = r#"
    fn test() -> i32 {
      let x = 0
      {
        let y = 1 + x(7, z)
      }
      {
        ret 8
      }
    }
  "#;

  let ast = JangGrammar::parse_fallible(lex_stream(code.chars())).unwrap();

  write_ast!("{}", ast);
}
