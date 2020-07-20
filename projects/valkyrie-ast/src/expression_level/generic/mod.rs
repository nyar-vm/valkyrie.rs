mod display;

use super::*;
use crate::ArgumentTermNode;

/// `class A⦓T: S = K⦔` or `class A<T: S = K>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, ExpressionTypeNode, ExpressionTypeNode>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `A⦓T⦔` or `A::<T>` or `A(G: T)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCall {
    pub base: ExpressionTypeNode,
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, ExpressionTypeNode>>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl GenericCall {
    pub fn rebase(mut self: Box<Self>, base: ExpressionBody) -> Box<Self> {
        self.base.body = base;
        self
    }
}
