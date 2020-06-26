#![feature(lazy_cell)]
#![feature(const_trait_impl)]

pub mod expression;
pub mod helpers;
pub mod import;
pub mod number;
pub mod operators;
pub mod repl;
pub mod string;
mod suffix_call;
pub mod symbol;
pub mod table;
mod traits;

pub use crate::traits::ThisParser;
