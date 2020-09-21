use super::*;
#[cfg(feature = "pretty-print")]
mod display;

/// `guard ... else { ... }`
///
/// The else block must use control.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuardStatement {
    pub condition: GuardPattern,
    pub body: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}

/// `guard case ...`
#[derive(Clone, Debug, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GuardPattern {
    Case(Box<ImplicitCaseNode>),
    Inline(Box<ExpressionNode>),
}
