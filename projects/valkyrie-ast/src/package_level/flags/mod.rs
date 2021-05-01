use super::*;
use crate::ModifiedNode;
mod display;
mod iters;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagKind {
    /// aka. `enumerate`,
    Exclusive,
    /// aka. `flags`
    Juxtapose,
}

/// a number that encodes special semantics
///
/// `enumerate Bit(8bits): Trait { FlagA, FlagB }`
///
/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagDeclaration {
    /// The documentation for this flag.
    pub documentation: DocumentationNode,
    /// The kind of the flag statement
    pub kind: FlagKind,
    /// The name of the flag.
    pub name: IdentifierNode,
    /// The modifiers for this flag.
    pub modifiers: ModifiersNode,
    /// `(8bits)`
    pub layout: Option<ExpressionNode>,
    /// `: Trait`
    pub implements: Vec<String>,
    /// `{ FlagA, FlagB }`
    pub body: Vec<FlagTerm>,
    /// The range of the node.
    pub span: Range<u32>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagTerm {
    Encode(EncodeDeclaration),
    Method(MethodDeclaration),
}

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EncodeDeclaration {
    /// The documentation for this field.
    pub documentation: DocumentationNode,
    /// The identifier of the field.
    pub name: IdentifierNode,
    /// The value of the field if exists.
    pub value: Option<ExpressionType>,
    /// The range of the node.
    pub span: Range<u32>,
}
