use super::*;
mod display;

/// always equivalent to a statement that returns `( )`, and cannot be used as an `rvalue`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlNode {
    pub r#type: ControlType,
    pub expression: Option<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `resume DivideZero()`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RaiseNode {
    pub expression: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    /// `resume DivideZero()`
    Resume,
    /// `yield return?`
    YieldReturn,
    /// `yield break`
    YieldBreak,
    /// `yield from`
    YieldFrom,
}

impl ControlType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ControlType::Break => "break",
            ControlType::Continue => "continue",
            ControlType::Fallthrough => "fallthrough",
            ControlType::Return => "return",
            ControlType::Resume => "resume",
            ControlType::YieldReturn => "yield",
            ControlType::YieldBreak => "yield break",
            ControlType::YieldFrom => "yield from",
        }
    }
}
