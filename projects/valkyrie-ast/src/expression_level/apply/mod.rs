#[cfg(feature = "pretty-print")]
mod display;
use super::*;

/// `(mut self, a, b: int, c: T = 3, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<ApplyArgumentTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyArgumentTerm {
    pub term: ArgumentTermNode<ArgumentKeyNode, ExpressionNode, ExpressionNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentKeyNode {
    pub modifiers: Vec<IdentifierNode>,
    pub key: IdentifierNode,
}

/// `term.call(0, a: 1, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyDotNode {
    /// The raw string of the number.
    pub caller: IdentifierNode,
    /// The range of the number.
    pub terms: Vec<ApplyCallTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `apply(0, a: 1, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode {
    /// The raw string of the number.
    pub terms: Vec<ApplyCallTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallTerm {
    pub term: CallTermNode<IdentifierNode, ExpressionNode>,
}

impl<K, V> CallTermNode<K, V> {
    pub fn map_key<F, O>(self, f: F) -> CallTermNode<O, V>
    where
        F: FnOnce(K) -> O,
    {
        CallTermNode { key: self.key.map(f), value: self.value }
    }
    pub fn map_value<F, O>(self, f: F) -> CallTermNode<K, O>
    where
        F: FnOnce(V) -> O,
    {
        CallTermNode { key: self.key, value: f(self.value) }
    }
}

impl<K, V, D> ArgumentTermNode<K, V, D> {
    pub fn map_key<F, O>(self, f: F) -> ArgumentTermNode<O, V, D>
    where
        F: FnOnce(K) -> O,
    {
        ArgumentTermNode { key: f(self.key), value: self.value, default: self.default }
    }
    pub fn map_value<F, O>(self, f: F) -> ArgumentTermNode<K, O, D>
    where
        F: FnOnce(V) -> O,
    {
        ArgumentTermNode { key: self.key, value: self.value.map(f), default: self.default }
    }
    pub fn map_default<F, O>(self, f: F) -> ArgumentTermNode<K, V, O>
    where
        F: FnOnce(D) -> O,
    {
        ArgumentTermNode { key: self.key, value: self.value, default: self.default.map(f) }
    }
}
