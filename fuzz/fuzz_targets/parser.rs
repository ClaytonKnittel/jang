#![no_main]

use jang::parser::parse_jang;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  if let Ok(code) = str::from_utf8(data)
    && let Ok(ast) = parse_jang(code.chars())
  {
    std::hint::black_box(ast);
  }
});
