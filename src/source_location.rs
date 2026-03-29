#[derive(Clone)]
pub struct SourceLocation {
  pos: usize,
}

impl SourceLocation {
  pub fn new(pos: usize) -> Self {
    Self { pos }
  }

  pub fn pos(&self) -> usize {
    self.pos
  }
}
