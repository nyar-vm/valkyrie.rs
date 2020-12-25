use super::*;

mod display;

/// `@tail_call(ret, recursion: true)`, **MIR**
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TailCallNode {
    /// Weather it is a recursive tail call
    pub recursion: bool,
}

/// always equivalent to a statement that returns `( )`, and cannot be used as an `rvalue`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlNode {
    /// The type of control flow
    pub r#type: ControlType,
    /// The label of the control flow
    pub expression: Option<ExpressionNode>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `raise DivideZero()`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RaiseNode {
    /// The raised expression
    pub expression: Option<ExpressionType>,
    /// The range of the node
    pub span: Range<u32>,
}

/// The control flow keywords
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlType {
    /// `goto label`, can't construct from frontend
    Goto,
    /// `break label`, equivalent to `call/cc`
    Break,
    /// `continue label`, equivalent to `goto`
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
            Self::Goto => "goto",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Fallthrough => "fallthrough",
            Self::Return => "return",
            Self::Resume => "resume",
            Self::YieldReturn => "yield",
            Self::YieldBreak => "yield break",
            Self::YieldFrom => "yield from",
        }
    }
}
