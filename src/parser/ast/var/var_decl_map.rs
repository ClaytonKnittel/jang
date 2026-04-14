use std::collections::{HashMap, hash_map::Entry};

use crate::{
  error::{JangError, JangResult},
  parser::{
    ast::{
      id::{
        builder::IdBuilder,
        def::{AstDeclId, AstGlobalDeclId, AstLocalDeclId},
      },
      var::{
        var_decl::{GlobalDecl, LocalDecl},
        var_ref::VarRef,
      },
    },
    token::ident::Ident,
  },
};

enum GlobalDeclState {
  Unresolved,
  Resolved,
}

struct GlobalDeclInfo {
  id: AstGlobalDeclId,
  state: GlobalDeclState,
}

#[derive(Default)]
pub struct VarDeclMap {
  globals: HashMap<Ident, GlobalDeclInfo>,
  locals: Vec<HashMap<Ident, AstLocalDeclId>>,
}

impl VarDeclMap {
  pub fn enter_block_scope(&mut self) {
    self.locals.push(HashMap::new());
  }

  pub fn exit_block_scope(&mut self) {
    debug_assert!(!self.locals.is_empty());
    self.locals.pop();
  }

  /// Returns a global decl ID for a given declaration of that global
  /// identifier. Either returns a new ID, or an error if the ID could not be
  /// created (e.g. duplicate identifier).
  pub fn new_global_decl(
    &mut self,
    ident: Ident,
    id_builder: &mut IdBuilder,
  ) -> JangResult<GlobalDecl> {
    match self.globals.entry(ident.clone()) {
      Entry::Occupied(mut entry) => {
        if matches!(entry.get().state, GlobalDeclState::Resolved) {
          return Err(JangError::duplicate_ident(entry.key()));
        }

        entry.get_mut().state = GlobalDeclState::Resolved;
        Ok(GlobalDecl::new(entry.get().id, ident))
      }
      Entry::Vacant(entry) => {
        let id = id_builder.new_global_id();
        entry.insert(GlobalDeclInfo {
          id,
          state: GlobalDeclState::Resolved,
        });
        Ok(GlobalDecl::new(id, ident))
      }
    }
  }

  /// Returns a local decl ID for a local variable. If a local with this name
  /// already exists, its ID will be overwritten and all future references to
  /// this identifier will resolve to this variable.
  pub fn new_local_decl(&mut self, ident: Ident, id_builder: &mut IdBuilder) -> LocalDecl {
    debug_assert!(!self.locals.is_empty());
    let local_scope = self
      .locals
      .last_mut()
      .expect("Cannot create a local variable in the global scope");

    let new_id = id_builder.new_local_id();
    local_scope.insert(ident.clone(), new_id);
    LocalDecl::new(new_id, ident)
  }

  fn lookup_var_ref(&mut self, ident: &Ident) -> Option<AstDeclId> {
    self
      .locals
      .iter()
      .rev()
      .find_map(|local_map| local_map.get(ident).cloned().map(AstDeclId::Local))
      .or_else(|| {
        self
          .globals
          .get(ident)
          .map(|info| AstDeclId::Global(info.id))
      })
  }

  /// Returns the ID corresponding to a given var ref. This may insert a new
  /// placeholder global decl ID if no such variable is found, as globals may
  /// be used before they are declared.
  pub fn var_ref(&mut self, ident: Ident, id_builder: &mut IdBuilder) -> VarRef {
    if let Some(id) = self.lookup_var_ref(&ident) {
      return VarRef::from_id(id, ident);
    }

    debug_assert!(self.lookup_var_ref(&ident).is_none());
    let id = id_builder.new_global_id();
    self.globals.insert(
      ident.clone(),
      GlobalDeclInfo {
        id,
        state: GlobalDeclState::Unresolved,
      },
    );

    GlobalDecl::new(id, ident).into()
  }
}
