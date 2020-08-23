use super::*;

/// `when a > 0 && a < 10`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternCondition {
    /// The post condition of the pattern
    pub condition: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}
