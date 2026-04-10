use crate::{
  error::{JangError, JangResult},
  parser::{ast::jang_file::JangFile, grammar::JangGrammar, lexer::lex_stream},
};
use parser_generator::{error::ParserError, parser::Parser};

pub mod ast;
pub mod grammar;
pub mod lexer;
pub(crate) mod token;

pub fn parse_jang(stream: impl IntoIterator<Item = char>) -> JangResult<JangFile> {
  JangGrammar::parse_fallible(lex_stream(stream)).map_err(|err| match err {
    ParserError::UserError(err) => err,
    ParserError::ParseError { next_token } => {
      JangError::Grammar(ParserError::ParseError { next_token })
    }
    #[cfg(debug_assertions)]
    ParserError::OverlappingTokenMatchers { token } => {
      JangError::Grammar(ParserError::OverlappingTokenMatchers { token })
    }
  })
}
