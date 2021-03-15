#[cfg(feature = "pretty-print")]
mod display;

use super::*;

/// The kind of the parameter node
pub enum ParameterKind {
    /// `a: Type = null`
    Expression,
    /// `T: Trait = ()`
    Generic,
}

/// `class A⦓T: S = K⦔` or `class A::(T: S = K)`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterArgument {
    /// The kind of the parameter node
    pub kind: ParameterKind,
    /// The raw string of the number.
    pub terms: Vec<ParameterTerm>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `T: Type = type_expression`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterTerm {
    pub key: IdentifierNode,
    pub value: Option<ExpressionNode>,
    pub default: Option<ExpressionNode>,
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

/// `T: Type + Trait`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericCallTerm {
    /// Typed generic call term
    pub term: CallTermNode<IdentifierNode, ExpressionNode>,
}
