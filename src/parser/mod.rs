use std::error::Error;

use parser_generator::parser::ParserNoContext;

use crate::{
  error::JangResult,
  parser::{ast::jang_file::JangFile, grammar::JangGrammar, lexer::try_lex_stream},
};

pub mod ast;
pub mod grammar;
pub mod lexer;
pub(crate) mod token;

pub fn lex_and_parse_jang_file<E: Error>(
  text: impl IntoIterator<Item = Result<char, E>>,
) -> JangResult<JangFile> {
  Ok(JangGrammar::parse_fallible(try_lex_stream(text))?)
}
