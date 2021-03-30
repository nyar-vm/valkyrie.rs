use super::*;
use crate::StatementBlock;

mod display;

/// `(a + b, c: d, ..e)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentsList {
    /// The raw string of the number.
    pub terms: Vec<TupleTermNode>,
}

/// `#annotation mut this: null`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentTerm {
    /// The modifier conditions
    pub modifiers: ModifiersNode,
    /// The key of the argument
    pub key: ArgumentKey,
    /// The value of the argument
    pub value: ExpressionNode,
}

/// The key of the argument
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArgumentKey {
    /// `a + b`
    Nothing,
    /// `key: a + b`
    Symbol(IdentifierNode),
}

/// `f(a + b, c: d, ..e) { a + b }`
///
/// ```v
/// f { a + b }
/// f(0, key: 1, ..list)
/// f(0, key: 1, ..list) { a + b }
///
/// this.m { a + b }
/// this.m(0, key: 1, ..list)
/// this.m(0, key: 1, ..list) { a + b }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode {
    /// Weather it is a monadic call
    pub monadic: bool,
    /// The caller of argument
    pub caller: ExpressionType,
    /// The raw string of the number.
    pub arguments: Option<ArgumentsList>,
    /// The raw string of the number.
    pub body: Option<StatementBlock>,
    /// The range of the number.
    pub span: Range<u32>,
}

impl ValkyrieNode for ApplyCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl ApplyCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { caller: base, ..self }
    }
}
