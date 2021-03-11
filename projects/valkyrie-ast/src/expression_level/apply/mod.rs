use super::*;

#[cfg(feature = "pretty-print")]
mod display;

/// `(mut self, a, b: int, c: T = 3, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyArgument {
    /// The raw string of the number.
    pub terms: Vec<ApplyArgumentTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `mut self: T? = null, ⁑args, ⁂kwargs`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyArgumentTerm {
    pub term: ArgumentTermNode<ArgumentKeyNode, ExpressionNode, ExpressionNode>,
}

/// `mod1 mod2 args`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentKeyNode {
    pub modifiers: ModifiersNode,
    pub key: IdentifierNode,
}

/// `apply(0, a: 1, ⁑args, ⁂kwargs)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode {
    /// The call basement
    pub base: ExpressionType,
    /// Weather it is a monadic call
    pub monadic: bool,
    /// The caller of apply
    pub caller: ApplyCaller,
    /// The raw string of the number.
    pub arguments: Option<ApplyCallTerms>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// The key of tuple
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ApplyCaller {
    /// `object()`
    None,
    /// `object.1()`
    Integer(BigUint),
    /// `object.method()`
    Internal(IdentifierNode),
}

/// `(1, 2, 3, name: value, ..list)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallTerms {
    /// item
    pub terms: Vec<ApplyCallItem>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `0, a: 1, ..args, ...kwargs`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallItem {
    /// The modifier conditions
    pub modifiers: ModifiersNode,
    /// Parameters filled in according to fields
    pub parameter: Option<IdentifierNode>,
    /// The main body
    pub body: ExpressionType,
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
        Self { base, ..self }
    }
}
