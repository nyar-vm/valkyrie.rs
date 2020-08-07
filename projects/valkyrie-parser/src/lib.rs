#![feature(lazy_cell)]
#![feature(const_trait_impl)]

mod atomic;
mod conditional;
mod control_flow;
mod declarations;
pub mod expression;
pub mod helpers;
pub mod import;
pub mod number;
pub mod operators;
pub mod statements;
pub mod string;
mod suffix_call;
pub mod table;
mod traits;
pub(crate) mod utils;

pub use crate::{
    statements::{ReplRoot, ScriptRoot},
    traits::ThisParser,
};
