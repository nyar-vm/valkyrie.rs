use super::*;
#[cfg(feature = "pretty-print")]
mod display;

/// `if a {1} else if b {2} else if c {3} else {4}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStatement {
    pub branches: Vec<IfConditionNode>,
    pub else_branch: ElseStatement,
    /// The range of the node
    pub span: Range<u32>,
}

/// `switch { when a > 0: a, else: 0}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchStatement {
    pub branches: Vec<IfConditionNode>,
    pub else_branch: ElseStatement,
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ElseStatement {
    pub statements: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl IfStatement {
    /// Make the if statement into equivalent switch statement
    pub fn as_switch(&self) -> SwitchStatement {
        SwitchStatement { branches: self.branches.clone(), else_branch: self.else_branch.clone(), span: self.span.clone() }
    }
}

impl ConditionType {
    pub fn is_empty(&self) -> bool {
        match self {
            ConditionType::AlwaysTrue => true,
            _ => false,
        }
    }
}
