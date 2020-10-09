use super::*;
use crate::LetBindNode;
#[cfg(feature = "pretty-print")]
mod display;

/// `if a {1} else if b {2} else if c {3} else {4}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStatement {
    pub branches: Vec<IfConditionNode>,
    pub else_body: Option<ElseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `if let Some(a) = b then {...} else {...}`
///
///
/// ```vk
/// if let
///     Some(a) = b
/// then {
///
/// }
/// else {
///
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfLetStatement {
    /// The range of the node
    pub pattern: LetBindNode,
    /// The range of the node
    pub then_body: StatementBlock,
    /// The range of the node
    pub else_body: Option<ElseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `a > 0 then { ... }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfConditionNode {
    pub condition: ExpressionNode,
    pub body: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}

/// Helper function to format the body of an if statement
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElseStatement {
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// Helper function to format the body of an if statement
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenStatement {
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl IfStatement {
    /// Make the if statement into equivalent switch statement
    pub fn as_switch(&self) -> SwitchStatement {
        todo!()
    }
}

impl WhileConditionNode {
    pub fn is_empty(&self) -> bool {
        match self {
            WhileConditionNode::Unconditional => true,
            _ => false,
        }
    }
}
