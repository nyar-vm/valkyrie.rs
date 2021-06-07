use super::*;

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

#[derive(Clone, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnionTerm {
    /// `@procedural`
    Macro(ProceduralNode),
    /// `Variant { }`
    Variant(VariantDeclaration),
    /// `method()`
    Method(MethodDeclaration),
}

/// `Variant { }`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantDeclaration {
    /// The name of variants
    pub name: IdentifierNode,
    /// The annotations of the variant declaration.
    pub annotations: AnnotationNode,
    /// The main body of the variant
    pub body: Vec<ClassTerm>,
    /// The range of the node
    pub span: Range<u32>,
}
