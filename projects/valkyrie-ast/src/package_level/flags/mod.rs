use super::*;
mod display;


/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagsDeclaration {
    /// The documentation for this flag.
    pub documentation: DocumentationNode,
    /// `flags Name`
    pub namepath: NamePathNode,
    /// The modifiers for this flag.
    pub modifiers: Vec<IdentifierNode>,
    /// `(8bits)`
    pub layout: Option<ExpressionNode>,
    /// `: Trait`
    pub implements: Vec<String>,
    /// `{ FlagA, FlagB }`
    pub body: StatementBlock,
    /// The range of the node.
    pub span: Range<u32>,
}

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagsFieldDeclaration {
    /// The documentation for this field.
    pub documentation: DocumentationNode,
    /// The identifier of the field.
    pub name: IdentifierNode,
    /// The value of the field if exists.
    pub value: Option<ExpressionNode>,
    /// The range of the node.
    pub span: Range<u32>,
}
