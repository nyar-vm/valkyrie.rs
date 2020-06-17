use crate::{expression_level::symbol::IdentifierNode, utils::small_range};
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
mod arithmetic;
pub mod generic;
pub mod number;
pub mod operators;
pub mod string;
pub mod symbol;
pub mod table;
pub mod tuple;
pub mod view;
