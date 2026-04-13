use std::collections::HashMap;

use crate::{
  interpreter::{
    bytecode::runtime::local_table::LocalId,
    error::{InterpreterError, InterpreterResult},
  },
  parser::ast::{bind_statement::Mutability, id::def::AstLocalDeclId},
};

#[derive(Debug)]
pub struct IdentInfo {
  local_id: LocalId,
  mutability: Mutability,
}

impl IdentInfo {
  pub fn local_id(&self) -> LocalId {
    self.local_id
  }

  pub fn mutability(&self) -> &Mutability {
    &self.mutability
  }
}

#[derive(Debug, Default)]
pub struct JitCompilerLexicalScope {
  parent: Option<Box<JitCompilerLexicalScope>>,
  next_local_id: LocalId,
  locals: HashMap<AstLocalDeclId, IdentInfo>,
}

impl JitCompilerLexicalScope {
  pub fn get_binding(&self, local: AstLocalDeclId) -> Option<&IdentInfo> {
    self
      .locals
      .get(&local)
      .or_else(move || self.parent.as_ref()?.get_binding(local))
  }

  pub fn bind(&mut self, local: AstLocalDeclId, mutability: Mutability) -> LocalId {
    debug_assert!(!self.locals.contains_key(&local));

    let local_id = self.next_local_id;
    self.next_local_id = local_id.next();
    self.locals.insert(
      local,
      IdentInfo {
        local_id,
        mutability,
      },
    );

    local_id
  }

  pub fn enter_block(self) -> Self {
    let next_local_id = self.next_local_id;
    Self {
      parent: Some(Box::new(self)),
      next_local_id,
      locals: HashMap::new(),
    }
  }

  pub fn exit_block(self) -> InterpreterResult<Self> {
    self
      .parent
      .map(|b| *b)
      .ok_or_else(|| InterpreterError::jit_err("tried to exit from top-level scope"))
  }
}
