#[cfg(feature = "pretty-print")]
mod display;

use super::*;

/// `class A⦓T: S = K⦔` or `class A::(T: S = K)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgument {
    /// The raw string of the number.
    pub terms: Vec<GenericArgumentTerm>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `A⦓T⦔` or `A::(T)` or `A(G: T)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallNode {
    /// The raw string of the number.
    pub terms: Vec<GenericCallTerm>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `T: Type = type_expression`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericArgumentTerm {
    /// Typed generic argument term
    pub term: ArgumentTermNode<IdentifierNode, ExpressionNode, ExpressionNode>,
}

/// `T: Type + Trait`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallTerm {
    /// Typed generic call term
    pub term: CallTermNode<IdentifierNode, ExpressionNode>,
}
