use std::{collections::HashMap, rc::Rc};

use crate::{interpreter::bytecode::local_table::LocalId, parser::token::ident::Ident};

#[derive(Clone)]
pub struct JitCompilerLexicalScope<'a>(Rc<JitCompilerLexicalBlock<'a>>);

struct JitCompilerLexicalBlock<'a> {
  parent: Option<JitCompilerLexicalScope<'a>>,
  next_local_id: LocalId,
  locals: HashMap<&'a Ident, LocalId>,
}

impl<'a> JitCompilerLexicalScope<'a> {
  pub fn new() -> Self {
    Self(Rc::new(JitCompilerLexicalBlock {
      parent: None,
      next_local_id: LocalId::default(),
      locals: HashMap::new(),
    }))
  }

  pub fn get_binding(&self, name: &Ident) -> Option<LocalId> {
    self
      .0
      .locals
      .get(name)
      .copied()
      .or_else(|| self.0.parent.as_ref()?.get_binding(name))
  }

  pub fn bind(&self, name: &'a Ident) -> (Self, LocalId) {
    let local_id = self.0.next_local_id;
    let mut locals = self.0.locals.clone();
    locals.insert(name, local_id);

    (
      Self(Rc::new(JitCompilerLexicalBlock {
        parent: self.0.parent.clone(),
        next_local_id: local_id.next(),
        locals,
      })),
      local_id,
    )
  }

  pub fn push_block(&self) -> Self {
    Self(Rc::new(JitCompilerLexicalBlock {
      parent: Some(self.clone()),
      next_local_id: self.0.next_local_id,
      locals: HashMap::new(),
    }))
  }
}
