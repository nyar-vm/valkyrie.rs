use super::*;

#[cfg(feature = "pretty-print")]
mod display;

/// `while cond {...} else {...}`
///
///
/// ```vk
/// let $loop_else = false;
/// @!label(loop_1_start)
/// loop {
///     if !cond {
///         goto loop_1_end;
///     }
///     $loop_else = true;
///     "run main body"
/// }
/// @!label(loop_1_end)
/// if !$loop_else {
///    "run else body"
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoop {
    /// The kind of while loop, including `while` and `until`
    pub kind: WhileLoopKind,
    /// The condition of the loop
    pub condition: WhileConditionNode,
    /// The main body of the loop
    pub then_body: StatementBlock,
    /// If the loop does not execute once, execute this statement.
    pub else_body: Option<ElseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

/// The kind of while loop, including `while` and `until`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WhileLoopKind {
    /// `while condition {}`
    While,
    /// `while !condition {}`
    Until,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WhileConditionNode {
    /// `true` in while loop and `false` in until loop
    Unconditional,
    Case,
    Expression(Box<ExpressionNode>),
}

/// `otherwise { ... }`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OtherwiseStatement {
    pub terms: Vec<StatementNode>,
    /// The range of the node
    pub span: Range<u32>,
}
