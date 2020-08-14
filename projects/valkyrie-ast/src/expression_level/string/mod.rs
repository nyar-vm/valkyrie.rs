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

/// `t"template string with {slot + 1:X?}"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringTemplateNode {
    /// The raw string of the number.
    pub items: Vec<StringTemplateType>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `f"string %d %s %f"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringFormatter {
    pub items: Vec<StringTemplateType>,
    pub span: Range<u32>,
}

/// `qq"template with quasi {% quote %}"`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringQuasiQuote {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StringTemplateType {
    Pure(Box<StringLiteralNode>),
    Slot(Box<ExpressionNode>),
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
