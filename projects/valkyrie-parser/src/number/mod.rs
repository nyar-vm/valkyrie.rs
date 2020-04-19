mod display;
mod parser;

use std::ops::Range;
use crate::symbol::ValkyrieIdentifier;

/// A number literal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieNumber {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<ValkyrieIdentifier>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieBytes {
    /// The raw string of the number.
    pub value: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<ValkyrieIdentifier>,
    /// The range of the number.
    pub range: Range<usize>,
}
