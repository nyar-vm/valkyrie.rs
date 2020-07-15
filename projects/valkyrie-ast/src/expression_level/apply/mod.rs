mod display;
use super::*;

/// `term.call(0, a: 1, ⁑args, ⁂kwargs)`
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

/// `apply(0, a: 1, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode<E> {
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `term` or `field: term`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyTermNode<K, V> {
    pub key: Option<K>,
    pub value: V,
}

/// `def f(mut self, a, b: int, c: T = 3, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyArgumentNode<E1, E2> {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<ArgumentKeyNode, E1, E2>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `function(0, a: Type, b: Integer = 1)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentTermNode<K, V, D> {
    pub key: K,
    pub value: Option<V>,
    pub default: Option<D>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentKeyNode {
    pub modifiers: Vec<IdentifierNode>,
    pub name: IdentifierNode,
}

impl<E1, E2> ApplyArgumentNode<E1, E2> {
    pub fn map_value<F, T>(self, f: F) -> ApplyArgumentNode<T, E2>
    where
        F: FnOnce(E1) -> T + Copy,
    {
        let terms = self
            .terms
            .into_iter()
            .map(|term| ArgumentTermNode { key: term.key, value: term.value.map(f), default: term.default })
            .collect();
        ApplyArgumentNode { terms, range: self.range }
    }
}

impl<E> ApplyCallNode<E> {
    pub fn rebase(mut self: Box<Self>, base: E) -> Box<Self> {
        self.base = base;
        self
    }
}

impl<E> ApplyDotNode<E> {
    pub fn rebase(mut self: Box<Self>, base: E) -> Box<Self> {
        self.base = base;
        self
    }
}
