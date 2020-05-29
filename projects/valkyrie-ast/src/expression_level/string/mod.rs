use super::*;

mod display;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct StringLiteralNode {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the number.
    pub range: Range<usize>,
}
