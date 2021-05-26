use super::*;
mod display;
mod iters;

/// The flags kind
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
    /// The name of the flag.
    pub name: IdentifierNode,
    /// The kind of the flag statement
    pub kind: FlagKind,
    /// The annotations of this flag.
    pub annotations: AnnotationNode,
    /// `(8bits)`
    pub layout: Option<ExpressionNode>,
    /// `: Trait`
    pub implements: Vec<String>,
    /// `{ FlagA, FlagB }`
    pub body: Vec<FlagTerm>,
    /// The range of the node.
    pub span: Range<u32>,
}
/// Valid terms in the flags statement
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlagTerm {
    /// `Name = number`
    Encode(EncodeDeclaration),
    /// `method() { }`
    Method(MethodDeclaration),
}

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EncodeDeclaration {
    /// The documentation for this field.
    pub documentation: DocumentationList,
    /// The identifier of the field.
    pub name: IdentifierNode,
    /// The value of the field if exists.
    pub value: Option<ExpressionKind>,
    /// The range of the node.
    pub span: Range<u32>,
}
