use std::{error::Error, fs::File, io::BufReader, process::ExitCode};

use clap::Parser;
use jang::{
  error::{JangError, JangResult},
  interpreter::Interpreter,
  parser::lex_and_parse_jang_file,
};
use utf8_chars::BufReadCharsExt;

#[derive(Parser, Debug)]
struct Args {
  /// Name of the Jang file to run
  #[arg(short, long)]
  input: String,
}

fn run_program<E: Error>(text: impl IntoIterator<Item = Result<char, E>>) -> JangResult<i32> {
  let ast = lex_and_parse_jang_file(text)?;
  let interp = Interpreter::new(&ast)?;
  interp.run_main().map_err(JangError::Interpret)
}

fn main() -> ExitCode {
  let args = Args::parse();

  let file = match File::open(&args.input) {
    Ok(file) => file,
    Err(err) => {
      println!("Unable to open file {}: {}", args.input, err);
      return ExitCode::FAILURE;
    }
  };

  match run_program(BufReader::new(file).chars()) {
    Ok(result) => {
      println!("Program returned {result}");
      ExitCode::SUCCESS
    }
    Err(err) => {
      println!("Error: {err}");
      ExitCode::FAILURE
    }
  }
}
