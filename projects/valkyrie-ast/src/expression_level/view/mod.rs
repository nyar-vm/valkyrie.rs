#[cfg(feature = "pretty-print")]
mod display;
use super::*;
/// `array⁅index0⁆` or `array[index1]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptNode {
    pub index0: bool,
    /// The raw string of the number.
    pub terms: Vec<SubscriptTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `⁅index⁆` or `⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubscriptTermNode {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Index(ExpressionNode),
    /// `array[start:end:step]`
    Slice(SubscriptSliceNode),
}

/// `⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptSliceNode {
    /// The raw string of the number.
    pub start: Option<ExpressionNode>,
    /// The unit of the number, if any.
    pub end: Option<ExpressionNode>,
    /// The unit of the number, if any.
    pub step: Option<ExpressionNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

impl SubscriptNode {
    pub fn method(&self) -> &'static str {
        if self.index0 { "subscript0" } else { "subscript1" }
    }
}
