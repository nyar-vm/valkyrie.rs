use super::*;
use crate::utils::take_range;
mod display;

#[derive(Clone, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNode {
    pub value: String,
    pub unit: Option<IdentifierNode>,
    pub span: Range<u32>,
}

impl NumberNode {
    pub fn get_range(&self) -> Range<usize> {
        take_range(&self.span)
    }
}

impl NumberNode {
    pub fn new<S: ToString>(text: S, range: &Range<usize>) -> NumberNode {
        NumberNode { value: text.to_string(), unit: None, span: small_range(range) }
    }
}
