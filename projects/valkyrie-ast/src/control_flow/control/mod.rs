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
    pub kind: ControlKind,
    /// In theory, all jumps need a destination
    pub label: String,
    /// The label of the control flow
    pub expression: Option<ExpressionType>,
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
pub enum ControlKind {
    /// `goto ^label`, equivalent to `call/cc`, can't construct from frontend
    Goto,
    /// `raise ^label`
    Raise,
    /// `break ^label`
    Break,
    /// `continue ^label`, equivalent to `goto`
    Continue,
    /// `fallthrough ^label`
    Fallthrough,
    /// `fallthrough! ^label`
    FallthroughUnchecked,
    /// `return ^label`
    Return,
    /// `resume ^label`
    Resume,
    /// `yield ^label`
    YieldReturn,
    /// `yield break ^label`
    YieldBreak,
    /// `yield from ^label`
    YieldFrom,
}
impl ValkyrieNode for RaiseNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
