use super::*;
mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallNode<E> {
    pub base: ExpressionBody,
    pub rest: E,
    pub span: Range<u32>,
}

/// `term` or `field: term`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallTermNode<K, V> {
    pub key: Option<K>,
    pub value: V,
}

/// `a: Integer = 1`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentTermNode<K, V, D> {
    pub key: K,
    pub value: Option<V>,
    pub default: Option<D>,
}