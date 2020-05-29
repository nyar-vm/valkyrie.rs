use super::*;
use crate::utils::take_range;

mod display;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct StringLiteralNode {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub span: Range<usize>,
}

impl StringLiteralNode {
    pub fn new<S: ToString>(value: S, range: Range<usize>) -> Self {
        Self { value: value.to_string(), unit: None, span: range }
    }
    pub fn with_unit(mut self, unit: IdentifierNode) -> Self {
        self.unit = Some(unit);
        self
    }
}
