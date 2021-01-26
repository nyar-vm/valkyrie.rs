use super::*;

mod display;

// pub enum TraitKind {
//     Trait,
// }

/// `trait name: Debug {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitDeclaration {
    pub name: IdentifierNode,
    pub fields: Vec<ClassFieldDeclaration>,
    pub methods: Vec<ClassMethodDeclaration>,
}

// pub enum TraitKind {
//     Interface,
// }
