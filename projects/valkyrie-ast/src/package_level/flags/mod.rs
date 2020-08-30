use super::*;

/// `flags Bit(8bits): Trait { FlagA, FlagB }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagsDeclaration {
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<StatementNode>,
    pub span: Range<u32>,
}

/// `Name = 0x00`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagFieldDeclaration {
    pub documentation: DocumentationNode,
    pub name: IdentifierNode,
    pub value: Option<ExpressionNode>,
    pub span: Range<u32>,
}
