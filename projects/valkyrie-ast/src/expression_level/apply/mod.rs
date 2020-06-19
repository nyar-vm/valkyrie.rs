mod display;
use super::*;

/// `<BASE>.f(0, a: 1, **args, ***kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyDotNode<E> {
    /// The raw string of the number.
    pub base: E,
    /// The raw string of the number.
    pub caller: IdentifierNode,
    /// The range of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `<BASE>(0, a: 1, **args, ***kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode<E> {
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyTermNode<K, V> {
    pub key: Option<K>,
    pub value: V,
}

/// `def f(mut self, a, b: int, c: T = 3, **args, ***kwargs)`
pub struct ApplyArgumentNode<E1, E2> {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, E1, E2>>,
    /// The range of the number.
    pub range: Range<usize>,
}
