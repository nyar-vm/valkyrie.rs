use super::*;

mod display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableKind {
    /// `(a, b, c)`
    Tuple,
    /// `{a, b, c}`
    OffsetTable,
    /// `{a = 1, b = 2, c = 3}`
    OrdinalTable,
}

/// A number literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableNode<E> {
    pub kind: TableKind,
    /// The raw string of the number.
    pub terms: Vec<TableTermNode<E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableTermNode<E> {
    /// `array[index]`, also can be a call_index `array[[1, 2, 3]]`
    Item(E),
    /// `a[start:end:step]`
    Pair(PairNode<IdentifierNode, E>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PairNode<K, V> {
    pub key: K,
    pub value: V,
}
