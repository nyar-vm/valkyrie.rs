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

/// `micro f(t: Type = default)` or `class A⦓T: Trait = Default⦔`
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
pub enum ParameterTerm {
    /// `<`
    ///
    /// The type on the left cannot be called by name, but the type on the right can be called by name.
    LMark,
    /// `>`
    ///
    /// The type on the left can be called by name, and the type on the right must be called by name.
    RMark,
    /// `#annotation mod a: Type = default`
    Single { modifiers: ModifiersNode, key: IdentifierNode, typing: Option<ExpressionNode>, default: Option<ExpressionNode> },
    /// `#annotation mod ..list: Type`
    UnpackList { modifiers: ModifiersNode, key: IdentifierNode, typing: Option<ExpressionNode> },
    /// `#annotation mod ...dict: Type`
    UnpackDict { modifiers: ModifiersNode, key: IdentifierNode, typing: Option<ExpressionNode> },
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
