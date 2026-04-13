use std::error::Error;

use parser_generator::parser::Parser;

use crate::{
  error::JangResult,
  parser::{
    ast::{JangAst, builder_context::AstBuilderContext, jang_file::JangFile},
    grammar::JangGrammar,
    lexer::try_lex_stream,
  },
};

pub mod ast;
pub mod grammar;
pub mod lexer;
pub(crate) mod token;

pub fn lex_and_parse_jang_file<E: Error>(
  text: impl IntoIterator<Item = Result<char, E>>,
) -> JangResult<JangAst> {
  let mut ctx = AstBuilderContext::default();
  let jang_file = JangGrammar::parse_fallible_with_ctx(try_lex_stream(text), &mut ctx)?;

  Ok(JangAst::new(jang_file, ctx.build()))
}
