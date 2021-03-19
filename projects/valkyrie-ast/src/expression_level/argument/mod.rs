use super::*;
use crate::StatementBlock;

mod display;

/// `(a + b, c: d, ..e)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentsList {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTerm>,
    /// The range of the number.
    pub span: Range<u32>,
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

impl ApplyCallNode {
    /// Replace placeholder with actual expression
    pub fn with_base(self, base: ExpressionType) -> Self {
        Self { caller: base, ..self }
    }
}
