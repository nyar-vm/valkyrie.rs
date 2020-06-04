#![feature(lazy_cell)]
#![feature(const_trait_impl)]

pub mod call_apply;
pub mod call_dot;
pub mod call_index;
pub mod expression;
pub mod helpers;
pub mod import;
pub mod number;
pub mod operators;
pub mod repl;
pub mod string;
pub mod symbol;
pub mod table;
mod traits;

pub use crate::traits::ThisParser;
