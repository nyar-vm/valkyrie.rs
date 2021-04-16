use super::*;

impl crate::DefineUnionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<UnionDeclaration> {
        Success {
            value: UnionDeclaration {
                document: Default::default(),
                modifiers: Default::default(),
                name: IdentifierNode { name: "".to_string(), span: Default::default() },
                layout: None,
                derive_traits: vec![],
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}

impl KwUnionNode {
    // pub fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
