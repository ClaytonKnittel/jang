use cknittel_util::peekable_stream::{IntoPeekableStream, PeekableStream};

use crate::{
  error::{JangError, JangResult},
  parser::token::{
    JangToken,
    ident::Ident,
    keyword::Keyword,
    literal::NumericLiteral,
    operator::{Op, Operator, Spacing, is_op},
  },
  source_location::SourceLocation,
};

fn is_ident_char(ch: &char) -> bool {
  matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn is_numeric_char(ch: &char) -> bool {
  matches!(ch, '0'..='9' | '.')
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

  fn collect_while<F: FnMut(&char) -> bool>(&mut self, first_char: char, mut cond: F) -> String {
    let mut string_val = String::from(first_char);
    while let Some(next_char) = self.char_iter.peek()
      && cond(&next_char)
    {
      string_val.push(next_char.take());
    }
    string_val
  }

  fn parse_ident_or_keyword(&mut self, first_char: char) -> JangResult<JangToken> {
    let ident = self.collect_while(first_char, is_ident_char);
    if let Some(keyword) = Keyword::build_from_string(&ident) {
      Ok(keyword.into())
    } else {
      Ok(Ident::new(ident).into())
    }
  }

  fn parse_numeric(&mut self, first_char: char) -> JangToken {
    let numeric = self.collect_while(first_char, is_numeric_char);
    NumericLiteral::from_str(numeric).into()
  }

  fn parse_operator(&mut self, first_char: char) -> JangToken {
    let spacing = if self.char_iter.peek().as_deref().cloned().is_some_and(is_op) {
      Spacing::Joint
    } else {
      Spacing::Alone
    };
    Operator::new(
      Op::from_char(first_char)
        .expect("parse_operator should only be called on operator characters"),
      spacing,
    )
    .into()
  }

  fn parse_next(&mut self) -> JangResult<Option<JangToken>> {
    self.consume_all_whitespace();
    match self.char_iter.next() {
      Some(first_char @ ('a'..='z' | 'A'..='Z' | '_')) => {
        Ok(Some(self.parse_ident_or_keyword(first_char)?))
      }
      Some(first_char @ ('0'..='9')) => Ok(Some(self.parse_numeric(first_char))),
      Some('.') => {
        if self
          .char_iter
          .peek()
          .as_deref()
          .is_some_and(char::is_ascii_digit)
        {
          Ok(Some(self.parse_numeric('.')))
        } else {
          Ok(Some(self.parse_operator('.')))
        }
      }
      Some(first_char @ ('=' | ',' | '(' | ')' | '-' | '<' | '>' | ':')) => {
        Ok(Some(self.parse_operator(first_char)))
      }
      Some(ch) => Err(JangError::parse_error(
        format!("Unexpected symbol '{ch}'"),
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
    joint_operator, keyword, operator,
    parser::{
      lexer::lex_stream,
      token::{
        ident::matchers::ident,
        literal::matchers::{float, integral},
      },
    },
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
  fn test_all_keywords() {
    let text = "fn let ret";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![keyword!(Function), keyword!(Let), keyword!(Ret)]
    );
  }

  #[gtest]
  fn test_integral_literal() {
    let text = "123";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![integral("123")]);
  }

  #[gtest]
  fn test_float_literal() {
    let text = "1.3 0.56 .92 8. .0 0.";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![
        float("1.3"),
        float("0.56"),
        float(".92"),
        float("8."),
        float(".0"),
        float("0.")
      ]
    );
  }

  #[gtest]
  fn test_lone_dot() {
    // This should not parse as a numeric literal.
    let text = ".";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(Dot)]));
  }

  #[gtest]
  fn test_joint_dots() {
    let text = "..";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(
      tokens,
      ok(elements_are![joint_operator!(Dot), operator!(Dot)])
    );
  }

  #[gtest]
  fn test_dots_with_space() {
    let text = ". .";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(Dot), operator!(Dot)]));
  }

  #[gtest]
  fn test_other_operators() {
    let text = "= , ( ) - < > : .";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(
      tokens,
      ok(elements_are![
        operator!(Equal),
        operator!(Comma),
        operator!(OpenParen),
        operator!(CloseParen),
        operator!(Dash),
        operator!(LessThan),
        operator!(GreaterThan),
        operator!(Colon),
        operator!(Dot)
      ])
    );
  }

  #[gtest]
  fn test_unexpected_char() {
    let text = "#";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, err(pat![JangError::ParseError(anything())]));
  }
}
