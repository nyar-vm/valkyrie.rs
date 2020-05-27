use lispify::Lispify;
use std::fmt::{Display, Formatter};

mod display;
mod parser;

use std::ops::Range;
use valkyrie_ast::IdentifierNode;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieString {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieTemplate {
    /// The raw string of the number.
    pub bytes: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieString {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.unit.eq(&other.unit)
    }
}

impl PartialEq for ValkyrieTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.unit.eq(&other.unit)
    }
}
