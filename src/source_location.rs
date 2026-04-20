use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceLocation {
  line: usize,
  col: usize,
}

impl SourceLocation {
  pub fn file_start() -> Self {
    Self { line: 0, col: 0 }
  }

  #[cfg(test)]
  pub fn new(line: usize, col: usize) -> Self {
    Self { line, col }
  }

  pub fn advance_char(&mut self) -> Self {
    self.col += 1;
    *self
  }

  pub fn advance_line(&mut self) -> Self {
    self.line += 1;
    self.col = 0;
    *self
  }
}

impl Display for SourceLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "line {}, col {}", self.line, self.col)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
  pos: SourceLocation,
  len: usize,
}

impl SourceSpan {
  #[cfg(test)]
  pub fn new(pos: SourceLocation, len: usize) -> Self {
    Self { pos, len }
  }

  pub fn join(&mut self, span: impl Into<SourceSpan>) {
    let span = span.into();
    if self.pos.line == span.pos.line {
      let min_col = self.pos.col.min(span.pos.col);
      let max_col = self.pos.col.max(span.pos.col);
      self.pos.col = min_col;
      self.len = max_col - min_col + 1;
    } else {
      unimplemented!("Can't join spans from different lines yet");
    }
  }
}

impl From<SourceLocation> for SourceSpan {
  fn from(value: SourceLocation) -> Self {
    Self { pos: value, len: 1 }
  }
}

impl Display for SourceSpan {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}, len {}", self.pos, self.len)
  }
}
