use super::*;
#[cfg(feature = "pretty-print")]
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
    pub literal: String,
    /// The unit of the number, if any.
    pub handler: Option<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl StringLiteralNode {
    pub fn new<S: ToString>(value: S, start: u32, end: u32) -> Self {
        Self { literal: value.to_string(), handler: None, span: start..end }
    }
    pub fn as_raw(&self) -> StringTextNode {
        StringTextNode { text: self.literal.clone(), span: self.span.clone() }
    }
    /// Attack a handler to the unit of the number.
    pub fn with_handler(self, handler: IdentifierNode) -> Self {
        Self { handler: Some(handler), ..self }
    }
}
