use crate::parser::{
  ast::expression::Expression,
  token::{ident::Ident, literal::Literal},
};

pub struct JangAst {
  literals: Vec<Literal>,
  idents: Vec<Ident>,
  expressions: Vec<Expression>,
}
