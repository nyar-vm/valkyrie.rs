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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArrayKind {
    /// [1, 1]
    Ordinal,
    /// [1, 1]
    Offset,
}

impl Default for ArrayKind {
    fn default() -> Self {
        Self::Ordinal
    }
}

/// `(table, ), (named: tuple, expression)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleNode {
    ///  The kind of table.
    pub kind: TupleKind,
    /// The raw string of the number.
    pub terms: Vec<TupleTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `[0, [], [:], [::]]`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayNode {
    ///  The kind of table.
    pub kind: ArrayKind,
    /// Terms
    pub terms: Vec<ArrayTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `⁅index⁆` or `⁅start : end : step⁆`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ArrayTermNode {
    /// The index kind
    Index {
        /// The index of range
        index: ExpressionType,
    },
    /// The range
    Range {
        /// The first element in range
        head: Option<ExpressionType>,
        /// The middle element in range
        tail: Option<ExpressionType>,
        /// The
        step: Option<ExpressionType>,
    },
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

impl ArrayNode {
    pub fn as_tuple(&self) -> Option<TupleNode> {
        let mut terms = Vec::with_capacity(self.terms.len());
        for term in &self.terms {
            terms.push(term.as_tuple()?)
        }
        Some(TupleNode { kind: TupleKind::List, terms: vec![], span: self.span.clone() })
    }
}

impl ArrayTermNode {
    pub fn as_tuple(&self) -> Option<TupleTermNode> {
        match self {
            ArrayTermNode::Index { index } => Some(TupleTermNode { pair: CallTermNode { key: None, value: index.clone() } }),
            ArrayTermNode::Range { .. } => None,
        }
    }
}
