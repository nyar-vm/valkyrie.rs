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
impl ValkyrieNode for StringTextNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl ValkyrieNode for StringLiteralNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl StringTextNode {
    /// Create a new raw text node
    pub fn new<S: ToString>(value: S, span: Range<u32>) -> Self {
        Self { text: value.to_string(), span }
    }
    /// Convert to an identifier
    pub fn as_identifier(&self) -> IdentifierNode {
        IdentifierNode { name: self.text.clone(), span: self.span.clone() }
    }
}

impl StringLiteralNode {
    /// Create a text node with unknown handler
    pub fn new<S: ToString>(value: S, start: u32, end: u32) -> Self {
        Self { literal: value.to_string(), handler: None, span: start..end }
    }
    /// Convert to a raw string
    pub fn as_raw(&self) -> StringTextNode {
        StringTextNode { text: self.literal.clone(), span: self.span.clone() }
    }
    pub fn as_escaped(&self) -> String {
        self.literal.clone()
    }

    /// Attack a handler to the unit of the number.
    pub fn with_handler(self, handler: IdentifierNode) -> Self {
        Self { handler: Some(handler), ..self }
    }
}
