mod display;
mod parser;

use std::ops::Range;
use bit_set::BitSet;
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
    pub bits: BitSet,
    /// The unit of the number, if any.
    pub unit: Option<ValkyrieIdentifier>,
    /// The range of the number.
    pub range: Range<usize>,
}
