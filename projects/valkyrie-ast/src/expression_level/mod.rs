use crate::{expression_level::table::ArgumentTermNode, utils::small_range, ApplyTermNode, IdentifierNode};
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
pub mod apply;
mod arithmetic;
pub mod generic;
pub mod number;
pub mod operators;
pub mod string;
pub mod symbol;
pub mod table;
pub mod view;
