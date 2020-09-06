#![feature(lazy_cell)]
#![feature(const_trait_impl)]

mod atomic;
mod conditional;
mod control_flow;
mod declarations;
mod expression;
mod helpers;
mod operators;
mod statements;
mod suffix_call;
mod table;
mod traits;
mod utils;

pub use crate::{statements::ReplRoot, traits::ThisParser};
pub use valkyrie_ast::ProgramRoot;
