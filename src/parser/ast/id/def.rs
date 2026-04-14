macro_rules! define_ast_ids {
  (
    $(
      $(#[$meta:meta])*
      $id_type:ident;
    )*
  ) => {
    $(
      $(#[$meta])*
      #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
      pub struct $id_type(pub (super) usize);
      impl crate::parser::ast::id::AstId for $id_type {}
      impl crate::parser::ast::id::AstIdImpl for $id_type {
        fn default() -> Self {
          Self(0)
        }

        fn next_id(&self) -> Self {
          Self(self.0 + 1)
        }
      }
    )*
  };
}

define_ast_ids!(
  /// ID to describe an expression in the AST.
  AstExpressionId;

  /// ID for a local name decl from a function parameter or local binding.
  AstLocalDeclId;

  /// ID for a global name decl.
  AstGlobalDeclId;
);

pub enum AstDeclId {
  Global(AstGlobalDeclId),
  Local(AstLocalDeclId),
}
