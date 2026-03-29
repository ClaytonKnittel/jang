use crate::{error::JangResult, parser::token::JangToken};

pub fn lex_stream<I: IntoIterator<Item = char>>(
  stream: I,
) -> impl Iterator<Item = JangResult<JangToken>> {
  std::iter::empty()
}
