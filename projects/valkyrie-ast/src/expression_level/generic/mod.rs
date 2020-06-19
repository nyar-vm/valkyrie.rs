use crate::{ApplyTermNode, IdentifierNode};
use std::ops::Range;

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
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}
