#![feature(lazy_cell)]
#![feature(const_trait_impl)]
// #![feature(once_cell)]
//
// pub mod grammar;
// pub mod utils;
//
// pub mod ast {
//     pub use nyar_hir::ast::*;
// }
//
// mod traits;
//
// pub use grammar::LexerContext;
// pub use nyar_hir::{NyarError, NyarErrorKind, Result};
// pub use traits::ASTDump;

pub mod call_apply;
pub mod call_dot;
pub mod call_index;
pub mod expression;
pub mod helpers;
pub mod number;
pub mod operators;
pub mod repl;
pub mod string;
pub mod symbol;
pub mod table;
