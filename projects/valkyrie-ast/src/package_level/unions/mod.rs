use super::*;

/// `union Bit(8bits): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionDeclaration {
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<StatementNode>,
}


/// `union Bit(8bits): Trait { VariantA, VariantB}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantDeclaration {
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: Vec<StatementNode>,
}
