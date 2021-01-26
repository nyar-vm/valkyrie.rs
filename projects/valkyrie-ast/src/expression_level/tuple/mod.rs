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
    /// element in tuple
    pub pair: CallTermNode<TupleKeyType, ExpressionType>,
}

/// The key of tuple
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TupleKeyType {
    /// A valid identifier key
    Identifier(Box<IdentifierNode>),
    /// A valid number key
    Number(Box<NumberLiteralNode>),
    /// A raw string key
    String(Box<StringLiteralNode>),
    /// A subscript key
    Subscript(Box<SubscriptCallNode>),
}

impl Default for TupleKind {
    fn default() -> Self {
        Self::Tuple
    }
}
