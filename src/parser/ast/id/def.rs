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

  /// ID for a name occuring in an expression or on the LHS of a rebind.
  /// Each name ref is associated with a AstLocalDeclId or a AstGlobalDeclId.
  AstNameRefExpressionId;
);
