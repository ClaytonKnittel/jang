pub mod error;
pub mod type_analysis;
mod type_checker;
mod type_inference_table;
pub mod typed_ast_id;
pub mod types;

pub use type_checker::*;
