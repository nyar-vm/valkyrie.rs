use super::*;

mod display;

/// `trait name: Debug {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitDeclaration {
    pub fields: Vec<ClassFieldDeclaration>,
    pub methods: Vec<ClassMethodDeclaration>,
}

// pub enum TraitKind {
//     Interface,
// }
