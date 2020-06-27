#![feature(lazy_cell)]
#![feature(const_trait_impl)]

mod conditional;
pub mod expression;
pub mod helpers;
pub mod import;
mod looping;
pub mod number;
pub mod operators;
pub mod repl;
pub mod string;
mod suffix_call;
pub mod symbol;
pub mod table;
mod traits;

pub use crate::traits::ThisParser;
