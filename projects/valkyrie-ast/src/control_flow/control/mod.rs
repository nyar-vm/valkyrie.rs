mod display;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlNode {
    pub r#type: ControlType,
    pub expression: Option<ExpressionNode>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlType {
    /// `break label`
    Break,
    /// `continue label`
    Continue,
    /// `fallthrough`
    Fallthrough,
    /// `return`
    Return,
    /// `raise`
    Raise,
    /// `yield return?`
    YieldReturn,
    /// `yield break`
    YieldBreak,
    /// `yield from`
    YieldFrom,
}
