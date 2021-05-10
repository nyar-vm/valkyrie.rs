use super::*;

#[cfg(feature = "pretty-print")]
mod display;

mod iters;

/// `union Bit(8bits): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaggedDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: ModifierList,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TaggedTerm {
    Variant(VariantDeclaration),
}

/// iterate over all variants in the union
#[derive(Clone, Debug)]
pub struct TaggedIterator<'a> {
    iter: core::slice::Iter<'a, StatementNode>,
}

/// `VariantA { field: Type = default }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The range of the number.
    pub variant: IdentifierNode,
    // pub modifiers: ModifiersNode,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub statements: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}
