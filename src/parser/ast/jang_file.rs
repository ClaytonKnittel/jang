use std::fmt::Display;

use cknittel_util::builder::Builder;

use crate::parser::ast::{function_decl::FunctionDecl, type_decl::TypeDecl};

#[derive(Clone, Debug, Builder)]
pub struct JangFile {
  #[vec]
  function_decls: Vec<FunctionDecl>,
  #[vec]
  type_decls: Vec<TypeDecl>,
}

impl JangFile {
  pub fn function_decls(&self) -> &[FunctionDecl] {
    &self.function_decls
  }

  pub fn type_decls(&self) -> &[TypeDecl] {
    &self.type_decls
  }
}

impl Display for JangFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for type_decl in &self.type_decls {
      write!(f, "{type_decl}")?;
    }
    for function_decl in &self.function_decls {
      write!(f, "{function_decl}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use crate::parser::ast::{function_decl::FunctionDecl, jang_file::JangFile, type_decl::TypeDecl};
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

  pub fn jang_file_with_types<'a>(
    type_decls_matcher: impl Matcher<&'a [TypeDecl]>,
  ) -> impl Matcher<&'a JangFile> {
    property!(&JangFile.type_decls(), type_decls_matcher)
  }

  pub fn jang_file_with_type<'a>(
    type_decl_matcher: impl Matcher<&'a TypeDecl> + 'a,
  ) -> impl Matcher<&'a JangFile> {
    jang_file_with_types(elements_are![type_decl_matcher])
  }
}
