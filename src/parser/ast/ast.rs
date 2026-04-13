use crate::parser::{
  ast::{id::node_map::NodeMap, jang_file::JangFile},
  token::ident::Ident,
};

pub struct JangAst {
  file: JangFile,
  id_table: IdTable,
}
