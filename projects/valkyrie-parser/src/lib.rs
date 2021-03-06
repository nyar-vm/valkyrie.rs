#![feature(lazy_cell)]
#![feature(const_trait_impl)]

mod codegen;

pub use crate::{codegen::*, helpers::ProgramContext};
mod atomic;
// mod control_flow;
// mod declarations;
mod expression;
mod helpers;
mod statements;
// mod string_like;
// mod suffix_call;
// mod table;
// mod traits;
// mod utils;
// mod validation;
//
// pub use crate::{statements::ReplRoot, traits::ThisParser};
// pub use valkyrie_ast::ProgramRoot;
