use std::{convert::Infallible, error::Error};

use cknittel_util::{
  peekable_stream::{IntoPeekableStream, PeekableStream},
  tuple::TupleTraits,
};

use crate::{
  error::{JangError, JangResult},
  parser::{
    char_iter::CharIter,
    token::{
      JangToken,
      ident::Ident,
      keyword::Keyword,
      literal::{Literal, NumericLiteral},
      operator::{Op, Operator},
    },
  },
  source_location::{SourceLocation, SourceSpan},
};

fn is_ident_char(ch: &char) -> bool {
  matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn is_numeric_char(ch: &char) -> bool {
  matches!(ch, '0'..='9' | '.')
}

struct TokenIter<E, I: Iterator<Item = Result<char, E>>> {
  char_iter: PeekableStream<CharIter<E, I>>,
  should_emit_joint: bool,
}

impl<E: Error, I: Iterator<Item = Result<char, E>>> TokenIter<E, I> {
  fn consume_all_whitespace(&mut self) {
    while self.char_iter.peek().as_deref().is_some_and(|res| {
      res
        .as_ref()
        .ok()
        .map(TupleTraits::first)
        .is_some_and(char::is_ascii_whitespace)
    }) {
      self.char_iter.next();
    }
  }

  fn peek_next_token(&mut self) -> Option<(char, SourceLocation)> {
    self
      .char_iter
      .peek()
      .as_deref()
      .map(Result::as_ref)
      .and_then(Result::ok)
      .cloned()
  }

  fn collect_while<F: FnMut(&char) -> bool>(
    &mut self,
    first_char: char,
    source_loc: SourceLocation,
    mut cond: F,
  ) -> (String, SourceSpan) {
    let mut string_val = String::from(first_char);
    let mut span = SourceSpan::from(source_loc);
    while let Some((next_char, next_loc)) = self.peek_next_token() {
      if cond(&next_char) {
        string_val.push(next_char);
        span.join(next_loc);
        self.next().expect("Cannot fail if `peek` returned `Some`");
      } else {
        break;
      }
    }
    (string_val, span)
  }

  fn parse_ident_or_keyword(&mut self, first_char: char, source_loc: SourceLocation) -> JangToken {
    let (ident, span) = self.collect_while(first_char, source_loc, is_ident_char);
    if let Some(keyword) = Keyword::build_from_string(&ident) {
      return keyword.into();
    }

    if matches!(self.peek_next_token(), Some(('(', _))) {
      // Identifiers join to open parenthesis, to disambiguate function calls
      // from an expression on one line that ends with an ident, followed by
      // an expression on the next line that starts with open parenthesis.
      self.should_emit_joint = true;
    }
    Ident::new(ident, span).into()
  }

  fn parse_numeric(
    &mut self,
    first_char: char,
    source_loc: SourceLocation,
  ) -> JangResult<JangToken> {
    let numeric = self
      .collect_while(first_char, source_loc, is_numeric_char)
      .first();

    // Numerics cannot be immediately followed by identifiers.
    if let Some((ch, loc)) = self.peek_next_token()
      && is_ident_char(&ch)
    {
      return Err(JangError::unexpected_symbol(ch, loc.into()));
    }

    Ok(Literal::from(NumericLiteral::from_str(numeric)).into())
  }

  fn parse_operator(&mut self, first_char: char) -> JangToken {
    let op = Op::from_char_with_peek(first_char, self.char_iter.peek());

    if self
      .peek_next_token()
      .is_some_and(|(next_char, _)| op.can_join(next_char))
    {
      self.should_emit_joint = true;
    }

    Operator::new(op).into()
  }

  fn next(&mut self) -> JangResult<Option<(char, SourceLocation)>> {
    self
      .char_iter
      .next()
      .transpose()
      .map_err(|err| JangError::ForeignError(format!("{err}")))
  }

  fn parse_next(&mut self) -> JangResult<Option<JangToken>> {
    self.consume_all_whitespace();

    let Some((ch, source_loc)) = self.next()? else {
      return Ok(None);
    };

    match ch {
      first_char @ ('a'..='z' | 'A'..='Z' | '_') => {
        Ok(Some(self.parse_ident_or_keyword(first_char, source_loc)))
      }
      first_char @ ('0'..='9') => Ok(Some(self.parse_numeric(first_char, source_loc)?)),
      '.' => {
        if self
          .peek_next_token()
          .is_some_and(|(token, _)| token.is_ascii_digit())
        {
          Ok(Some(self.parse_numeric('.', source_loc)?))
        } else {
          Ok(Some(self.parse_operator('.')))
        }
      }
      first_char @ ('=' | ',' | '(' | ')' | '{' | '}' | '-' | '<' | '>' | ':' | '+' | '*' | '/'
      | '%' | '|' | '!' | '&') => Ok(Some(self.parse_operator(first_char))),
      ch => Err(JangError::unexpected_symbol(ch, source_loc.into())),
    }
  }
}

impl<E: Error, I: Iterator<Item = Result<char, E>>> Iterator for TokenIter<E, I> {
  type Item = JangResult<JangToken>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.should_emit_joint {
      self.should_emit_joint = false;
      Some(Ok(JangToken::Joint))
    } else {
      self.parse_next().transpose()
    }
  }
}

pub fn try_lex_stream<E: Error, I: IntoIterator<Item = Result<char, E>>>(
  stream: I,
) -> impl Iterator<Item = JangResult<JangToken>> {
  TokenIter {
    char_iter: CharIter::new(stream.into_iter()).peekable_stream(),
    should_emit_joint: false,
  }
}

pub fn lex_stream<I: IntoIterator<Item = char>>(
  stream: I,
) -> impl Iterator<Item = JangResult<JangToken>> {
  TokenIter {
    char_iter: CharIter::new(stream.into_iter().map(Ok::<_, Infallible>)).peekable_stream(),
    should_emit_joint: false,
  }
}

#[cfg(test)]
mod tests {
  use cknittel_util::iter::CollectResult;
  use googletest::prelude::*;

  use crate::{
    error::{JangError, matchers::unexpected_symbol},
    keyword, operator,
    parser::{
      lexer::lex_stream,
      token::{
        ident::matchers::ident_token,
        literal::matchers::{float_token, integral_token},
        matchers::joint,
      },
    },
  };

  #[gtest]
  fn test_single_ident() {
    let text = "my_idenT";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![ident_token("my_idenT")]);
  }

  #[gtest]
  fn test_many_idents() {
    let text = "  my_\niden\t\t\n T";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![ident_token("my_"), ident_token("iden"), ident_token("T")]
    );
  }

  #[gtest]
  fn test_joint_ident() {
    let text = "lone ( joint(";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![
        ident_token("lone"),
        operator!(OpenParen),
        ident_token("joint"),
        joint(),
        operator!(OpenParen)
      ]
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
    expect_that!(
      tokens,
      elements_are![keyword!(Function), ident_token("my_fn")]
    );
  }

  #[gtest]
  fn test_keyword_requires_space() {
    // This should lex as a single ident, not a keyword followed by a literal.
    let text = "fn2";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![ident_token("fn2")]);
  }

  #[gtest]
  fn test_all_keywords() {
    let text = "fn let ret if else loop break type mut unit bool i32 i64 f32 f64";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![
        keyword!(Function),
        keyword!(Let),
        keyword!(Ret),
        keyword!(If),
        keyword!(Else),
        keyword!(Loop),
        keyword!(Break),
        keyword!(Type),
        keyword!(Mut),
        keyword!(Unit),
        keyword!(Bool),
        keyword!(I32),
        keyword!(I64),
        keyword!(F32),
        keyword!(F64),
      ]
    );
  }

  #[gtest]
  fn test_integral_literal() {
    let text = "123";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(tokens, elements_are![integral_token("123")]);
  }

  #[gtest]
  fn test_float_literal() {
    let text = "1.3 0.56 .92 8. .0 0.";

    let tokens = lex_stream(text.chars()).collect_result_vec().unwrap();
    expect_that!(
      tokens,
      elements_are![
        float_token("1.3"),
        float_token("0.56"),
        float_token(".92"),
        float_token("8."),
        float_token(".0"),
        float_token("0.")
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
  fn test_arrow() {
    let text = "->";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(RightArrow)]));
  }

  #[gtest]
  fn test_thiqq_arrow() {
    let text = "=>";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(ThiqqRightArrow)]));
  }

  #[gtest]
  fn test_equal_to() {
    let text = "==";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(DoubleEqual)]));
  }

  #[gtest]
  fn test_not_equal() {
    let text = "!=";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(NotEqual)]));
  }

  #[gtest]
  fn test_greater_than_equal() {
    let text = ">=";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(GreaterOrEqual)]));
  }

  #[gtest]
  fn test_less_than_equal() {
    let text = "<=";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(LessOrEqual)]));
  }

  #[gtest]
  fn test_double_ampersand() {
    let text = "&&";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(LogicalAnd)]));
  }

  #[gtest]
  fn test_double_bar() {
    let text = "||";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, ok(elements_are![operator!(LogicalOr)]));
  }

  #[gtest]
  fn test_joint_close_open_paren() {
    let text = ")(";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(
      tokens,
      ok(elements_are![
        operator!(CloseParen),
        joint(),
        operator!(OpenParen)
      ])
    );
  }

  #[gtest]
  fn test_arrow_with_space() {
    let text = "- >";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    let x = elements_are![operator!(Dash), operator!(GreaterThan)];
    expect_that!(tokens, ok(x));
  }

  #[gtest]
  fn test_other_operators() {
    let text = "= , ( ) { } - < > ! : . + * / % | &";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(
      tokens,
      ok(elements_are![
        operator!(Equal),
        operator!(Comma),
        operator!(OpenParen),
        operator!(CloseParen),
        operator!(OpenBracket),
        operator!(CloseBracket),
        operator!(Dash),
        operator!(LessThan),
        operator!(GreaterThan),
        operator!(Bang),
        operator!(Colon),
        operator!(Dot),
        operator!(Plus),
        operator!(Star),
        operator!(Slash),
        operator!(Percent),
        operator!(Bar),
        operator!(Ampersand),
      ])
    );
  }

  #[gtest]
  fn test_unexpected_char() {
    let text = "#";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, err(pat![JangError::Parse(anything())]));
  }

  #[gtest]
  fn test_numeric_prefixed_ident() {
    let text = "1e";

    let tokens = lex_stream(text.chars()).collect_result_vec();
    expect_that!(tokens, err(unexpected_symbol(&'e')));
  }
}
