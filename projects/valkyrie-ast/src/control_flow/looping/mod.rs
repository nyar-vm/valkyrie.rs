use super::*;
mod display;

/// `while cond {...} else {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoopNode {
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub r#else: Vec<StatementNode>,
    pub range: Range<usize>,
}

/// ```vk
/// for i in 0..10 {
///     ...
/// }
/// else {
///     ...
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForLoopNode {
    pub pattern: PatternType,
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub r#else: Vec<StatementNode>,
    pub range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConditionType {
    AlwaysTrue,
    Case,
    Expression(Box<ExpressionNode<ExpressionType>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternType {
    Tuple(Vec<ExpressionNode<ExpressionType>>),
    Case,
}
