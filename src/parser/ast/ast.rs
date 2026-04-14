use crate::parser::ast::{id::IdTable, jang_file::JangFile};

#[derive(Debug)]
pub struct JangAst {
  file: JangFile,
  id_table: IdTable,
}

impl JangAst {
  pub fn new(file: JangFile, id_table: IdTable) -> Self {
    Self { file, id_table }
  }

  pub fn file(&self) -> &JangFile {
    &self.file
  }

  pub fn id_table(&self) -> &IdTable {
    &self.id_table
  }
}
