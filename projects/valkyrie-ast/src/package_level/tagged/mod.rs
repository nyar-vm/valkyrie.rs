use super::*;
use alloc::string::ToString;

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
    pub modifiers: ModifiersNode,
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

/// `public static final synchronized class Main {}`
///
/// - Auxiliary parsing function, not instantiable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ModifiersNode {
    pub terms: Vec<IdentifierNode>,
}

impl ModifiersNode {
    pub fn new(modifiers: Vec<IdentifierNode>) -> Self {
        Self { terms: modifiers }
    }
    pub fn contains(&self, modifier: &str) -> bool {
        self.terms.iter().any(|x| x.name.eq(modifier))
    }
}
