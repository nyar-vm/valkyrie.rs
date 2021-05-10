use super::*;

impl crate::DefineTraitNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TraitDeclaration> {
        Success { value: TraitDeclaration { name: self.identifier.build(ctx), terms: vec![] }, diagnostics: vec![] }
    }
}

impl crate::KwTraitNode {
    // pub fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
