use crate::{
    expression_level::{
        string::StringLiteralNode,
        symbol::{IdentifierNode, NamepathNode},
    },
    utils::small_range,
    NumberLiteralNode, ValkyrieOperator,
};
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};
mod arithmetic;
pub mod number;
pub mod operators;
pub mod string;
pub mod symbol;
