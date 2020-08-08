mod display;
mod parser;

use crate::{traits::ThisParser, utils::get_span};
use lispify::{Lisp, ListString};
use std::{ops::Range, str::FromStr};
use valkyrie_ast::{IdentifierNode, StringLiteralNode};
use valkyrie_types::third_party::pex::{
    helpers::{make_from_str, quotation_pair, quotation_pair_nested, whitespace},
    ParseResult, ParseState, StopBecause,
};

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct StringTemplateNode {
    /// The raw string of the number.
    pub bytes: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for StringTemplateNode {
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.unit.eq(&other.unit)
    }
}
