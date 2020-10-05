use super::*;
use crate::{ArgumentKeyNode, OtherwiseStatement};
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
    pub then_body: StatementBlock,
    /// `otherwise { body }`
    pub else_body: Option<OtherwiseStatement>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `for ref a, mut b in {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForBarePattern {
    /// The bare tuple pattern
    pub pattern: Vec<ArgumentKeyNode>,
    /// The range of the node
    pub span: Range<u32>,
}

impl ForBarePattern {
    /// Convert this bare pattern into tuple pattern
    #[allow(clippy::wrong_self_convention)]
    pub fn as_pattern_expression(self) -> PatternExpressionNode {
        PatternExpressionNode::Tuple(self.pattern)
    }
}
