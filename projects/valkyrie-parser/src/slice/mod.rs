mod display;
mod parser;

use crate::{expression::ValkyrieExpression, symbol::ValkyrieIdentifier};
use std::ops::Range;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieSlice {
    /// The raw string of the number.
    pub terms: Vec<ValkyrieSliceTerm>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieSliceTerm {
    /// The raw string of the number.
    pub start: Option<ValkyrieExpression>,
    /// The unit of the number, if any.
    pub end: Option<ValkyrieExpression>,
    /// The unit of the number, if any.
    pub step: Option<ValkyrieExpression>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieSliceTerm {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.end.eq(&other.end)
    }
}

impl PartialEq for ValkyrieSlice {
    fn eq(&self, other: &Self) -> bool {
        self.terms.eq(&other.terms) && self.unit.eq(&other.unit)
    }
}
