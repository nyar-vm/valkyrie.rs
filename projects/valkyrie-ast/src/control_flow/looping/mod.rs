use super::*;

#[cfg(feature = "pretty-print")]
mod display;

/// `while cond {...} else {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoop {
    pub condition: ConditionType,
    pub body: FunctionBody,
    pub r#else: Option<ElsePart>,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternType {
    Tuple(Vec<ArgumentKeyNode>),
    Case,
}
