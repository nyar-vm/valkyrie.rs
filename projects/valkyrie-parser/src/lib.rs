#![feature(lazy_cell)]
#![feature(const_trait_impl)]

mod atomic;
mod conditional;
mod control_flow;
mod declarations;
mod expression;
mod helpers;
mod number;
mod operators;
mod statements;
mod string;
mod suffix_call;
mod table;
mod traits;
mod utils;

pub use crate::{
    statements::{ReplRoot, ScriptRoot},
    traits::ThisParser,
};
