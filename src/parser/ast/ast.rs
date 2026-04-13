use crate::parser::ast::{id::builder::IdTable, jang_file::JangFile};

pub struct JangAst {
  file: JangFile,
  id_table: IdTable,
}
