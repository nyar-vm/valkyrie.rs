use crate::{ApplyTermNode, IdentifierNode};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
};
mod display;

/// `class A<T: S = K>` or `class A⦓T: S = K⦔`
pub struct GenericArgumentNode<E> {
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `A::<T>` or `A⦓T⦔` or `A(G: T)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCall<E> {
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl<E> GenericCall<E> {
    pub fn rebase(mut self: Box<Self>, base: E) -> Box<Self> {
        self.base = base;
        self
    }
}
