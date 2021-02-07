use super::*;

#[cfg(feature = "pretty-print")]
mod display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TupleKind {
    /// `(a, b, ..c)`
    Tuple,
    /// `[a: 1, b, ..c]`
    List,
    /// `{a: 1, b, ..c}`
    Dict,
}

/// `(tuple, ), (named: tuple, expression)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleNode {
    ///  The kind of tuple.
    pub kind: TupleKind,
    /// The raw string of the number.
    pub terms: Vec<TupleTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `a: item`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleTermNode {
    pub key: TupleKeyType,
    pub value: ExpressionType,
}

/// The key of tuple
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TupleKeyType {
    /// This expression has no key
    Nothing,
    /// A valid identifier key, or a string key
    Identifier(IdentifierNode),
    /// A valid number key
    Number(BigUint),
    /// A subscript key
    Subscript(SubscriptCallNode),
}

impl Default for TupleKeyType {
    fn default() -> Self {
        Self::Nothing
    }
}
impl ValkyrieNode for TupleNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
impl Default for TupleKind {
    fn default() -> Self {
        Self::Tuple
    }
}
