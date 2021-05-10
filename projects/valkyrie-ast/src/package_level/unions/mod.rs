use super::*;

#[cfg(feature = "pretty-print")]
mod display;

mod iters;

/// `union Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionDeclaration {
    /// The annotations of this union
    pub annotations: AnnotationNode,
    /// The range of the number.
    pub name: IdentifierNode,
    pub layout: Option<String>,
    pub derive_traits: Vec<String>,
    pub terms: Vec<UnionTerm>,
    /// The text range of the statement
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnionTerm {
    Variant(VariantDeclaration),
    Method(MethodDeclaration),
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    pub modifiers: ModifierList,
    pub field_name: IdentifierNode,
    pub r#type: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}
