use super::*;

mod display;

/// Pure text of a string literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringTextNode {
    /// The unescaped text of the string.
    pub text: String,
    /// The range of the node
    pub span: Range<u32>,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringLiteralNode {
    /// The raw string of the number.
    pub raw: String,
    /// The unit of the number, if any.
    pub unit: Option<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl StringLiteralNode {
    pub fn new<S: ToString>(value: S, start: u32, end: u32) -> Self {
        Self { raw: value.to_string(), unit: None, span: start..end }
    }
    pub fn as_raw(&self) -> StringTextNode {
        StringTextNode { text: self.raw.clone(), span: self.span.clone() }
    }
    pub fn as_escaped(&self) -> String {
        self.to_string()
    }
    pub fn with_unit(mut self, unit: IdentifierNode) -> Self {
        self.unit = Some(unit);
        self
    }
}
