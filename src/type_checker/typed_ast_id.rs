use cknittel_util::from_variants::FromVariants;

use crate::parser::ast::{
  id::{
    adt::map::{IdMap, IdMapIterator},
    def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId},
  },
  jang_file::JangFile,
  var::var_ref::VarRef,
};

// AST IDs with an associated type.
#[derive(Clone, Copy, FromVariants)]
pub enum TypedAstId {
  Expression(AstExpressionId),
  GlobalDecl(AstGlobalDeclId),
  LocalDecl(AstLocalDeclId),
}

impl From<&VarRef> for TypedAstId {
  fn from(var_ref: &VarRef) -> Self {
    match var_ref {
      VarRef::Global(var_decl) => var_decl.id().into(),
      VarRef::Local(var_decl) => var_decl.id().into(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypedAstIdTable<T> {
  expressions: IdMap<AstExpressionId, T>,
  globals: IdMap<AstGlobalDeclId, T>,
  locals: IdMap<AstLocalDeclId, T>,
}

impl<T: std::clone::Clone> TypedAstIdTable<T> {
  pub fn new(jang_file: &JangFile) -> Self {
    Self {
      expressions: jang_file.new_expression_id_map(),
      globals: jang_file.new_global_decl_id_map(),
      locals: jang_file.new_local_decl_id_map(),
    }
  }

  pub fn get(&self, id: impl Into<TypedAstId>) -> Option<&T> {
    match id.into() {
      TypedAstId::Expression(id) => self.expressions.get(id),
      TypedAstId::GlobalDecl(id) => self.globals.get(id),
      TypedAstId::LocalDecl(id) => self.locals.get(id),
    }
  }

  pub fn insert(&mut self, id: impl Into<TypedAstId>, value: T) {
    match id.into() {
      TypedAstId::Expression(id) => self.expressions.insert(id, value),
      TypedAstId::GlobalDecl(id) => self.globals.insert(id, value),
      TypedAstId::LocalDecl(id) => self.locals.insert(id, value),
    }
  }
}

pub struct TypedAstIdTableIterator<T> {
  expressions: IdMapIterator<AstExpressionId, T>,
  globals: IdMapIterator<AstGlobalDeclId, T>,
  locals: IdMapIterator<AstLocalDeclId, T>,
}

impl<T> Iterator for TypedAstIdTableIterator<T> {
  type Item = (TypedAstId, T);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some((id, v)) = self.expressions.next() {
      Some((id.into(), v))
    } else if let Some((id, v)) = self.globals.next() {
      Some((id.into(), v))
    } else if let Some((id, v)) = self.locals.next() {
      Some((id.into(), v))
    } else {
      None
    }
  }
}

impl<T> IntoIterator for TypedAstIdTable<T> {
  type Item = (TypedAstId, T);
  type IntoIter = TypedAstIdTableIterator<T>;

  fn into_iter(self) -> Self::IntoIter {
    Self::IntoIter {
      expressions: self.expressions.into_iter(),
      globals: self.globals.into_iter(),
      locals: self.locals.into_iter(),
    }
  }
}
