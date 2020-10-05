use super::*;
use crate::{ArgumentKeyNode, ArgumentTermNode};
mod display;

/// `for ... in ... if ... {...} else {...}`
///
///
/// ```vk
/// let $loop_else = false;
/// let mut iterator = iter.into_iterator();
/// @!label(loop.1.start)
/// loop {
///     let $next = iterator.next();
///     if $next.is_none() {
///         goto loop.1.end;
///     }
///     let $pattern = @unchecked $next!;
///     if !condition {
///        goto loop.1.start;
///     }
///
///     $loop_else = true;
///     "run main body"
/// }
/// @!label(loop.1.end)
///
/// if !$loop_else {
///     "run else body"
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForLoop {
    /// `for pattern`
    pub pattern: PatternExpressionNode,
    /// `in iterator`
    pub iterator: ExpressionNode,
    /// `if condition`
    pub condition: Option<PatternGuard>,
    /// `{ body }`
    pub body: StatementBlock,
    /// `else { body }`
    pub no_run: Option<ElseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `for ref a, mut b in {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BareForPattern {
    pub pattern: Vec<ArgumentKeyNode>,
    pub span: Range<u32>,
}

impl BareForPattern {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_pattern_expression(self) -> PatternExpressionNode {
        PatternExpressionNode::Tuple(self.pattern)
    }
}
