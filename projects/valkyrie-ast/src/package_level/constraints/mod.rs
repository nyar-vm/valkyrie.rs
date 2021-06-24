use super::*;

mod display;

mod iters;

/// `class Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintDeclaration {
    /// The document of the class
    pub annotations: AnnotationNode,
    /// The parameter arguments of the class.
    pub generic: Option<ParametersList>,
    /// The super class of the class.
    pub base_classes: Option<String>,
    /// The traits that the class implements.
    pub auto_traits: Vec<String>,
    /// All fields declared in this block, exclude extensions.
    pub terms: Vec<ConstraintTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

/// Valid terms in the class statements
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintTerm {
    /// `@expand {}`
    Macro(ProceduralNode),
    /// `field: Type = default`
    Field(FieldDeclaration),
    /// `method()`
    Method(MethodDeclaration),
    /// `domain { }`
    Domain(DomainDeclaration),
}

impl ValkyrieNode for ConstraintDeclaration {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}
