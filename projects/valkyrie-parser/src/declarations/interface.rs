use super::*;

impl crate::DefineTraitNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TraitDeclaration> {
        Success {
            value: TraitDeclaration {
                name: IdentifierNode { name: "".to_string(), span: Default::default() },
                fields: vec![],
                methods: vec![],
            },
            diagnostics: vec![],
        }
    }
}

impl crate::KwTraitNode {
    // pub fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
