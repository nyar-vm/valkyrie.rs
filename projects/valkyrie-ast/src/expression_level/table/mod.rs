use super::*;
#[cfg(feature = "pretty-print")]
mod display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableKind {
    /// `(a, b, c)`
    Tuple,
    /// `[a, b, c]`
    OffsetTable,
    /// `{a = 1, b = 2, c = 3}`
    OrdinalTable,
}

/// `[table]` or `(tuple)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableNode {
    pub kind: TableKind,
    /// The raw string of the number.
    pub terms: Vec<TableTermNode>,
    /// The range of the number.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableTermNode {
    pub pair: CallTermNode<TableKeyType, ExpressionNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableKeyType {
    Identifier(Box<IdentifierNode>),
    Number(Box<NumberLiteralNode>),
    String(Box<StringLiteralNode>),
    Subscript(Box<SubscriptNode>),
}

impl Default for TableKind {
    fn default() -> Self {
        Self::Tuple
    }
}
