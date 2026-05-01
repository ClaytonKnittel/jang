use std::error::Error;

use parser_generator::parser::Parser;

use crate::{
  error::JangResult,
  parser::{
    ast::{builder_context::AstBuilderContext, jang_file::JangFile},
    grammar::JangGrammar,
    lexer::try_lex_stream,
  },
};

pub mod ast;
mod char_iter;
pub mod grammar;
pub mod lexer;
pub(crate) mod token;

pub fn lex_and_parse_jang_file<E: Error>(
  text: impl IntoIterator<Item = Result<char, E>>,
) -> JangResult<JangFile> {
  Ok(JangGrammar::parse_fallible_with_ctx(
    try_lex_stream(text),
    &mut AstBuilderContext::default(),
  )?)
}
