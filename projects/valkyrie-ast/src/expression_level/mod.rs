use crate::{
    expression_level::identifier::{IdentifierNode, NamepathNode},
    utils::small_range,
    ValkyrieOperator,
};
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

mod arithmetic;
pub mod decimal;
pub mod identifier;
pub mod operators;
