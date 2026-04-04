use std::fmt::Display;

use crate::parser::ast::{function_decl::FunctionDecl, type_decl::TypeDecl};

#[derive(Clone, Debug)]
pub struct JangFile {
  function_decls: Vec<FunctionDecl>,
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

#[derive(Clone, Debug, Default)]
pub struct JangFileBuilder {
  function_decls: Vec<FunctionDecl>,
  type_decls: Vec<TypeDecl>,
}

impl JangFileBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn push_function_decl(mut self, function_decl: FunctionDecl) -> JangFileBuilder {
    self.function_decls.push(function_decl);
    self
  }

  pub fn push_type_decl(mut self, type_decl: TypeDecl) -> JangFileBuilder {
    self.type_decls.push(type_decl);
    self
  }

  pub fn build(self) -> JangFile {
    JangFile {
      function_decls: self.function_decls,
      type_decls: self.type_decls,
    }
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
