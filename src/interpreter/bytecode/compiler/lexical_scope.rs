use std::collections::HashMap;

use crate::{
  interpreter::{
    bytecode::runtime::local_table::LocalId,
    error::{InterpreterError, InterpreterResult},
  },
  parser::{ast::assignment_statement::Mutability, token::ident::Ident},
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
pub struct JitCompilerLexicalScope<'a> {
  parent: Option<Box<JitCompilerLexicalScope<'a>>>,
  next_local_id: LocalId,
  locals: HashMap<&'a Ident, IdentInfo>,
}

impl<'a> JitCompilerLexicalScope<'a> {
  pub fn get_binding(&self, name: &Ident) -> Option<&IdentInfo> {
    self
      .locals
      .get(name)
      .or_else(|| self.parent.as_ref()?.get_binding(name))
  }

  pub fn bind(&mut self, name: &'a Ident, mutability: Mutability) -> LocalId {
    debug_assert!(!self.locals.contains_key(name));

    let local_id = self.next_local_id;
    self.next_local_id = local_id.next();
    self.locals.insert(
      name,
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
