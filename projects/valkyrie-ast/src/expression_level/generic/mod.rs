#[cfg(feature = "pretty-print")]
mod display;

use super::*;

/// `class A⦓T: S = K⦔` or `class A<T: S = K>`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgumentNode {
    /// The raw string of the number.
    pub terms: Vec<GenericArgumentTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// `A⦓T⦔` or `A::<T>` or `A(G: T)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallNode {
    /// The raw string of the number.
    pub terms: Vec<GenericCallTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgumentTerm {
    pub term: ArgumentTermNode<IdentifierNode, ExpressionNode, ExpressionNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallTerm {
    pub term: CallTermNode<IdentifierNode, ExpressionNode>,
}
