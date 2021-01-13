use super::*;

mod display;

/// `extends path::A: Debug {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendsStatement {
    pub fields: Vec<ClassFieldDeclaration>,
    pub methods: Vec<ClassMethodDeclaration>,
}
