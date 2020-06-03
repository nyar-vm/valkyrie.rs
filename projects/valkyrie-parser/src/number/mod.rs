mod display;
mod parser;
use crate::expression::ValkyrieExpression;
use lispify::LispNumber;
use pex::{
    helpers::{make_from_str, whitespace},
    ParseResult, ParseState, StopBecause,
};
use regex::Regex;
use std::{ops::Range, str::FromStr, sync::LazyLock};
use valkyrie_ast::IdentifierNode;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieBytes {
    /// The raw string of the number.
    pub bytes: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieBytes {
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.unit.eq(&other.unit)
    }
}
