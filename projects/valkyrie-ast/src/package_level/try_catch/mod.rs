use super::*;

/// `try T? { ... }.catch { ... }`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryStatementNode {
    pub body: ExpressionNode,
    pub catch: ExpressionNode,
}
