use std::fmt::Write;

pub struct CodeFormatter<F: std::io::Write> {
  formatter: F,
  spaces: Vec<u8>,
  indentation_level: usize,
  newline_queued: bool,
}

impl<F: std::io::Write> CodeFormatter<F> {
  const TAB_SPACES: usize = 2;

  pub fn new(formatter: F) -> Self {
    Self {
      formatter,
      spaces: vec![b'\n'],
      indentation_level: 0,
      newline_queued: false,
    }
  }

  fn cur_indentation_spaces_len(&self) -> usize {
    self.indentation_level * Self::TAB_SPACES + 1
  }

  fn spaces_and_formatter(&mut self) -> (&mut F, &str) {
    let spaces =
      unsafe { str::from_utf8_unchecked(&self.spaces[..self.cur_indentation_spaces_len()]) };
    (&mut self.formatter, spaces)
  }

  fn increment_indentation(&mut self) {
    if self.spaces.len() == self.cur_indentation_spaces_len() {
      self.spaces.extend([b' ', b' ']);
    }

    self.indentation_level += 1;
  }

  fn decrement_indentation(&mut self) {
    self.indentation_level -= 1;
  }
}

impl<F: std::io::Write> Drop for CodeFormatter<F> {
  fn drop(&mut self) {
    debug_assert_eq!(self.indentation_level, 0);
    if self.newline_queued {
      self
        .formatter
        .write_all(b"\n")
        .expect("Final write failed!");
    }
  }
}

impl<F: std::io::Write> Write for CodeFormatter<F> {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    let mut newline_queued = self.newline_queued;

    for line in s.split('\n') {
      if line.is_empty() {
        newline_queued = true;
        continue;
      } else if line.ends_with('}') {
        self.decrement_indentation();
      }

      let (f, spaces) = self.spaces_and_formatter();
      if newline_queued {
        f.write_all(spaces.as_bytes())
          .map_err(|_| std::fmt::Error)?;
      }

      f.write_all(line.as_bytes()).map_err(|_| std::fmt::Error)?;

      if line.ends_with('{') {
        self.increment_indentation();
      }

      newline_queued = false;
    }

    self.newline_queued = newline_queued;

    Ok(())
  }
}

#[macro_export]
macro_rules! write_ast {
  ($($arg:tt)*) => {
    <$crate::parser::ast::code_formatter::CodeFormatter<_> as ::std::fmt::Write>::write_fmt(
      &mut $crate::parser::ast::code_formatter::CodeFormatter::new(::std::io::stdout()),
      ::core::format_args!($($arg)*)
    )
    .unwrap()
  };
}
