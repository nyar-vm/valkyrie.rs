use crate::{expression_level::table::ArgumentTermNode, CallTermNode, IdentifierNode};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
};
mod display;

/// `def f(mut self, a, b: int, c: T = 3, **args, ***kwargs)`
pub struct ApplyArgumentNode<E1, E2> {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, E1, E2>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `f(0, a: 1, **args, ***kwargs)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApplyCallNode<E> {
    pub base: E,
    /// The raw string of the number.
    pub terms: Vec<CallTermNode<IdentifierNode, E>>,
    /// The range of the number.
    pub range: Range<usize>,
}
