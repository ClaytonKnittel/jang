#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![allow(clippy::module_inception)]

pub mod error;
pub mod interpreter;
pub mod parser;
mod source_location;
pub mod type_checker;
