mod display;

use super::*;
use crate::ArgumentTermNode;

/// `class A⦓T: S = K⦔` or `class A<T: S = K>`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<ArgumentTermNode<IdentifierNode, ExpressionNode, ExpressionNode>>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// `A⦓T⦔` or `A::<T>` or `A(G: T)`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCall {
    pub base: ExpressionNode,
    /// The raw string of the number.
    pub terms: Vec<ApplyTermNode<IdentifierNode, ExpressionNode>>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl Default for GenericArgumentNode {
    fn default() -> Self {
        Self { terms: Vec::new(), range: 0..0 }
    }
}

impl GenericCall {
    pub fn rebase(mut self: Box<Self>, base: ExpressionBody) -> Box<Self> {
        self.base.body = base;
        self
    }
}
