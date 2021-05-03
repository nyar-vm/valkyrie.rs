use super::*;

mod display;

/// `for ... in ... if ... {...}`
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
    pub pattern: PatternNode,
    /// `in iterator`
    pub iterator: ExpressionType,
    /// `if condition`
    pub condition: Option<ExpressionType>,
    /// `{ body }`
    pub then: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}
impl ValkyrieNode for ForLoop {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
/// `for ref a, mut b in {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForBarePattern {
    /// The bare tuple pattern
    pub pattern: Vec<ArgumentKey>,
    /// The range of the node
    pub span: Range<u32>,
}

impl ForBarePattern {
    /// Convert this bare pattern into tuple pattern
    #[allow(clippy::wrong_self_convention)]
    pub fn as_pattern_expression(self) -> PatternNode {
        TuplePatternNode {
            bind: None,
            name: None,
            terms: self.pattern.into_iter().map(PatternNode::from).collect(),
            span: self.span,
        }
        .into()
    }
}
