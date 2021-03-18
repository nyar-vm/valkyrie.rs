mod display;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberLiteralNode {
    pub value: String,
    pub unit: Option<IdentifierNode>,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for NumberLiteralNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl NumberLiteralNode {
    pub fn new<S: ToString>(text: S, span: Range<u32>) -> NumberLiteralNode {
        NumberLiteralNode { value: text.to_string(), unit: None, span }
    }
}
