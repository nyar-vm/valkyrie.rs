use super::*;

mod display;

/// `try T? { ... }.catch { ... }`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryStatement {
    /// The try block catch handler
    pub handler: Option<ExpressionNode>,
    /// The expression to run
    pub body: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}
