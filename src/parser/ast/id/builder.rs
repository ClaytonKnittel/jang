use crate::parser::ast::id::def::{AstExpressionId, AstGlobalDeclId, AstLocalDeclId};

macro_rules! define_ast_id_gen {
  (
    name: $ids_name:ident,
    context: $ctx_name:ident,

    $($(#[$meta:meta])* $id_type:ident => $field:ident, $method:ident;)*
  ) => {
    pub struct $ids_name {
      $($field: $id_type,)*
    }

    impl Default for $ids_name {
      fn default() -> Self {
        Self {
          $($field: $id_type(0),)*
        }
      }
    }

    impl $ids_name {
      $(
        fn $method(&mut self) -> $id_type {
          let id = self.$field;
          self.$field.0 += 1;
          id
        }
      )*
    }

    impl $ctx_name {
      $(
        pub fn $method(&mut self) -> $id_type {
          self.ids.$method()
        }
      )*
    }
  };
}

#[derive(Default)]
pub struct IdBuilder {
  ids: AstIds,
}

define_ast_id_gen!(
  name: AstIds,
  context: IdBuilder,

  // ID to describe an expression in the AST.
  AstExpressionId => expr_id, new_expr_id;

  // ID for local name declarations in a function parameter
  // or a local binding.
  AstLocalDeclId => local_decl_id, new_local_decl_id;

  // ID for global name declarations.
  AstGlobalDeclId => gbl_decl_id, new_global_decl_id;
);

pub enum AstDeclId {
  Global(AstGlobalDeclId),
  Local(AstLocalDeclId),
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::parser::ast::id::builder::IdBuilder;

  #[gtest]
  fn test_ast_expr_id() {
    let mut ctx = IdBuilder::default();
    let expr_id_a = ctx.new_expr_id();
    let expr_id_b = ctx.new_expr_id();
    expect_eq!(expr_id_a.0, 0);
    expect_eq!(expr_id_b.0, 1);
  }
}
