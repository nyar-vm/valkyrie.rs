mod display;
mod parser;

use std::ops::Range;
use crate::symbol::ValkyrieIdentifier;

pub struct ValkyrieNumber {
    pub raw: String,
    pub unit: Option<ValkyrieIdentifier>,
    pub range: Range<usize>,
}
