use super::*;

mod display;

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringLiteralNode {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

impl StringLiteralNode {
    pub fn new<S: ToString>(value: S, start: u32, end: u32) -> Self {
        Self { value: value.to_string(), unit: None, span: start..end }
    }
    pub fn with_unit(mut self, unit: IdentifierNode) -> Self {
        self.unit = Some(unit);
        self
    }
}
