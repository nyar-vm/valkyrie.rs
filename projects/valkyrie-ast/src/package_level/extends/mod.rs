use super::*;

mod display;

/// `extends path::A: Debug {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendsStatement {
    pub methods: Vec<ClassMethodDeclaration>,
}
