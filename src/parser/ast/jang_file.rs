use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::ast::function_decl::FunctionDecl;

#[derive(Clone, Debug, Builder)]
pub struct JangFile {
  function_decls: Vec<FunctionDecl>,
}

impl JangFile {
  pub fn function_decls(&self) -> &[FunctionDecl] {
    &self.function_decls
  }
}

impl Display for JangFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for function_decl in &self.function_decls {
      write!(f, "{function_decl}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{function_decl::FunctionDecl, jang_file::JangFile};
  use googletest::prelude::*;

  pub fn jang_file_functions<'a>(
    function_decls_matcher: impl Matcher<&'a [FunctionDecl]>,
  ) -> impl Matcher<&'a JangFile> {
    property!(&JangFile.function_decls(), function_decls_matcher)
  }

  pub fn jang_file_with_fn<'a>(
    function_decl_matcher: impl Matcher<&'a FunctionDecl> + 'a,
  ) -> impl Matcher<&'a JangFile> {
    jang_file_functions(elements_are![function_decl_matcher])
  }
}
