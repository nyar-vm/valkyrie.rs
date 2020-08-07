use super::*;

mod display;

// if a {1}
// if a {1} else {2}
// if a {1} else if b {2}
// if a {1} else if b {2} else {3}
// if a {1} else if b {2} else if c {3}
// if a {1} else if b {2} else if c {3} else {4}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStatementNode {
    pub branches: Vec<ConditionNode>,
    pub else_branch: Vec<StatementNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConditionNode {
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub range: Range<usize>,
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
    pub range: Range<usize>,
}

/// Helper function to format the body of an if statement
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ElsePart<'i> {
    pub body: Cow<'i, [StatementNode]>,
}
