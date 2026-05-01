use std::marker::PhantomData;

use crate::source_location::SourceLocation;

pub struct CharIter<E, I> {
  stream: I,
  pos: SourceLocation,
  _phantom: PhantomData<E>,
}

impl<E, I> CharIter<E, I> {
  pub fn new(stream: I) -> Self {
    Self {
      stream,
      pos: SourceLocation::file_start(),
      _phantom: PhantomData,
    }
  }

  fn advance_position(&mut self, ch: char) -> SourceLocation {
    let cur_pos = self.pos;
    if ch == '\n' {
      self.pos.advance_line();
    } else {
      self.pos.advance_char();
    }
    cur_pos
  }
}

impl<E, I> Iterator for CharIter<E, I>
where
  I: Iterator<Item = Result<char, E>>,
{
  type Item = Result<(char, SourceLocation), E>;

  fn next(&mut self) -> Option<Self::Item> {
    self
      .stream
      .next()
      .map(|result| result.map(|ch| (ch, self.advance_position(ch))))
  }
}
