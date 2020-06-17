use crate::{expression_level::table::ArgumentTermNode, CallTermNode, IdentifierNode};
use std::ops::Range;

/// `class A<T: S = K>` or `class A⦓T: S = K⦔`
pub struct TupleArgumentNode<E1, E2> {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, E1, E2>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `A(0, a: 1)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TupleCallNode<E> {
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<CallTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}
