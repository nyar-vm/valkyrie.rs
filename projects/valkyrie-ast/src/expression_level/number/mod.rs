use super::*;
mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberLiteralNode {
    pub value: String,
    pub unit: Option<IdentifierNode>,
    pub range: Range<usize>,
}

impl NumberLiteralNode {
    pub fn new<S: ToString>(text: S, range: &Range<usize>) -> NumberLiteralNode {
        NumberLiteralNode { value: text.to_string(), unit: None, range: range.clone() }
    }
}
