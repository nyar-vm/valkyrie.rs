use super::*;

impl crate::DefineUnionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<UnionDeclaration> {
        Success {
            value: UnionDeclaration {
                document: Default::default(),
                modifiers: Default::default(),
                name: self.identifier.build(ctx),
                layout: None,
                derive_traits: vec![],
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl crate::KwUnionNode {
    // pub fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
