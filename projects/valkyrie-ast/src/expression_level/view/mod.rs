use super::*;
mod display;

/// `array⁅index0⁆` or `array[index1]`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubscriptNode {
    pub index0: bool,
    /// The raw string of the number.
    pub base: ExpressionNode,
    /// The raw string of the number.
    pub terms: Vec<SubscriptTermNode>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `⁅index⁆` or `⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub range: Range<usize>,
}

impl SubscriptNode {
    pub fn rebase(mut self: Box<Self>, base: ExpressionBody) -> Box<Self> {
        self.base.body = base;
        self
    }
    pub fn method(&self) -> &'static str {
        if self.index0 { "subscript0" } else { "subscript1" }
    }
}

impl SubscriptTermNode {
    pub fn indexed(index: ExpressionNode) -> Self {
        Self::Index(index)
    }
    pub fn ranged(
        start: Option<ExpressionNode>,
        end: Option<ExpressionNode>,
        step: Option<ExpressionNode>,
        range: Range<usize>,
    ) -> Self {
        Self::Slice(SubscriptSliceNode { start, end, step, range })
    }
}
