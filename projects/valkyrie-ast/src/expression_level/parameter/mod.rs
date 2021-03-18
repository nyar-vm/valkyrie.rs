#[cfg(feature = "pretty-print")]
mod display;

use super::*;

/// The kind of the parameter node
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParameterKind {
    /// `a: Type = null`
    Expression,
    /// `T: Trait = ()`
    Generic,
}

/// `micro f(t: Type = default)` or `class F⦓T: Trait = Default⦔`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParametersList {
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
    /// `#annotation modifier a: Type = default`
    Single {
        /// The modifiers apply on the parameter
        modifiers: ModifiersNode,
        /// The name
        key: IdentifierNode,
        bound: Option<ExpressionNode>,
        default: Option<ExpressionNode>,
    },
    /// `#annotation modifier ..list: Type`
    UnpackList {
        /// The modifiers apply on the parameter
        modifiers: ModifiersNode,
        key: IdentifierNode,
        bound: Option<ExpressionNode>,
    },
    /// `#annotation modifier ...dict: Type`
    UnpackDict {
        /// The modifiers apply on the parameter
        modifiers: ModifiersNode,
        key: IdentifierNode,
        bound: Option<ExpressionNode>,
    },
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
    /// Typed parameter call term
    pub term: CallTermNode<IdentifierNode, ExpressionNode>,
}
