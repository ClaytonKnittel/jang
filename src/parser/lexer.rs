use cknittel_util::peekable_stream::{IntoPeekableStream, PeekableStream};

use crate::{
  error::{JangError, JangResult},
  parser::token::{Ident, JangToken, Keyword},
  source_location::SourceLocation,
};

fn is_ident_char(ch: &char) -> bool {
  matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

struct TokenIter<I: Iterator<Item = char>> {
  char_iter: PeekableStream<I>,
}

impl<I: Iterator<Item = char>> TokenIter<I> {
  fn consume_all_whitespace(&mut self) {
    while self
      .char_iter
      .peek()
      .as_deref()
      .is_some_and(char::is_ascii_whitespace)
    {
      self.char_iter.next();
    }
  }

  fn parse_ident_or_keyword(&mut self, first_char: char) -> JangResult<JangToken> {
    let mut ident = String::from(first_char);
    while let Some(next_char) = self.char_iter.peek()
      && is_ident_char(&next_char)
    {
      ident.push(next_char.take());
    }

    if let Some(keyword) = Keyword::build_from_string(&ident) {
      Ok(JangToken::Keyword(keyword))
    } else {
      Ok(JangToken::Ident(Ident::new(ident)))
    }
  }

  fn parse_next(&mut self) -> JangResult<Option<JangToken>> {
    self.consume_all_whitespace();
    match self.char_iter.next() {
      Some(first_char @ ('a'..='z' | 'A'..='Z' | '_')) => {
        Ok(Some(self.parse_ident_or_keyword(first_char)?))
      }
      Some(ch) => Err(JangError::parse_error(
        "Unexpected symbol {ch}",
        SourceLocation::new(0),
      )),
      None => Ok(None),
    }
  }
}

impl<I: Iterator<Item = char>> Iterator for TokenIter<I> {
  type Item = JangResult<JangToken>;

  fn next(&mut self) -> Option<Self::Item> {
    self.parse_next().transpose()
  }
}

pub fn lex_stream<I: IntoIterator<Item = char>>(
  stream: I,
) -> impl Iterator<Item = JangResult<JangToken>> {
  TokenIter {
    char_iter: stream.into_iter().peekable_stream(),
  }
}

#[cfg(test)]
mod tests {
  use cknittel_util::iter::CollectResult;
  use googletest::prelude::*;

  use crate::{
    error::JangError,
    keyword,
    parser::{lexer::lex_stream, test_util::ident},
  };

  #[gtest]
  fn test_single_ident() {
    let text = "my_idenT";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![ident("my_idenT")]);
  }

  #[gtest]
  fn test_many_idents() {
    let text = "  my_\niden\t\t\n T";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![ident("my_"), ident("iden"), ident("T")]
    );
  }

  #[gtest]
  fn test_single_keyword() {
    let text = "fn";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![keyword!(Function)]);
  }

  #[gtest]
  fn test_keyword_and_ident() {
    let text = "fn my_fn";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![keyword!(Function), ident("my_fn")]);
  }

  #[gtest]
  fn test_keyword_requires_space() {
    // This should lex as a single ident, not a keyword followed by a literal.
    let text = "fn2";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![ident("fn2")]);
  }

  #[gtest]
  fn test_unexpected_char() {
    let text = ".";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, err(pat![JangError::ParseError(anything())]));
  }
}
