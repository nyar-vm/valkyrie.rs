#[cfg(feature = "pretty-print")]
mod display;
use super::*;
// if a {1}
// if a {1} else {2}
// if a {1} else if b {2}
// if a {1} else if b {2} else {3}
// if a {1} else if b {2} else if c {3}
// if a {1} else if b {2} else if c {3} else {4}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStatement {
    pub branches: Vec<ConditionNode>,
    pub else_branch: ElseStatement,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConditionNode {
    pub condition: ConditionType,
    pub body: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConditionType {
    AlwaysTrue,
    Case,
    Expression(Box<ExpressionNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CasePatternNode {
    /// The range of the node
    pub span: Range<u32>,
}

/// Helper function to format the body of an if statement
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElseStatement {
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl ConditionType {
    pub fn is_empty(&self) -> bool {
        match self {
            ConditionType::AlwaysTrue => true,
            _ => false,
        }
    }
}
