#![no_main]

use jang::parser::grammar::testing::lex_and_parse_jang_file;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  if let Ok(code) = str::from_utf8(data)
    && let Ok(ast) = lex_and_parse_jang_file(code.chars())
  {
    std::hint::black_box(ast);
  }
});
