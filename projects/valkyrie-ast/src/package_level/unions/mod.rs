use super::*;
#[cfg(feature = "pretty-print")]
mod display;

mod iters;

/// `union Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionDeclaration {
    pub document: DocumentationNode,
    /// The range of the number.
    pub namepath: NamePathNode,
    pub modifiers: Vec<IdentifierNode>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub body: StatementBlock,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnionTerm {
    Field(UnionFieldDeclaration),
    Method(UnionMethodDeclaration),
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionFieldDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    pub modifiers: ModifiersNode,
    pub field_name: IdentifierNode,
    pub r#type: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `method()`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionMethodDeclaration {}

/// The iterator for [`UnionDeclaration`]
#[derive(Clone, Debug)]
pub struct UnionIterator<'a> {
    inner: core::slice::Iter<'a, StatementNode>,
}
