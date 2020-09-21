use super::*;

#[cfg(feature = "pretty-print")]
mod display;

/// `while cond {...} else {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoop {
    pub condition: WhileConditionNode,
    pub body: StatementBlock,
    pub r#else: Option<ElseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WhileConditionNode {
    AlwaysTrue,
    Case,
    Expression(Box<ExpressionNode>),
}
