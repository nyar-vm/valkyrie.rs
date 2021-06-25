use super::*;

mod display;

mod iters;

/// `class Name(Super): Trait {}`
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintDeclaration {
    /// The document of the class
    pub annotations: AnnotationNode,
    /// The parameter arguments of the class.
    pub generics: Vec<IdentifierNode>,
    /// All fields declared in this block, exclude extensions.
    pub terms: Vec<ConstraintTerm>,
}

impl Debug for ConstraintDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("Constraints");
        if !self.annotations.is_empty() {
            w.field("annotations", &self.annotations);
        }
        if !self.generics.is_empty() {
            w.field("generics", &self.generics);
        }
        if !self.terms.is_empty() {
            w.field("terms", &self.terms);
        }
        w.finish()
    }
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
impl Default for ConstraintDeclaration {
    fn default() -> Self {
        Self { annotations: Default::default(), generics: vec![], terms: vec![] }
    }
}

impl ConstraintDeclaration {
    pub fn is_empty(&self) -> bool {
        self.generics.is_empty() && self.terms.is_empty() && self.annotations.is_empty()
    }
}
