use cknittel_util::builder::Builder;

#[derive(Builder, Debug)]
pub struct NodeMap<ExprT, GlobalT, LocalT> {
  expressions: Vec<ExprT>,
  globals: Vec<GlobalT>,
  locals: Vec<LocalT>,
}
