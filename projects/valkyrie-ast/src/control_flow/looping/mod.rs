#[cfg(feature = "pretty-print")]
mod display;
use super::*;
/// `while cond {...} else {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoopNode {
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub r#else: Vec<StatementNode>,
    pub span: Range<u32>,
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
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternType {
    Tuple(Vec<ExpressionNode>),
    Case,
}
