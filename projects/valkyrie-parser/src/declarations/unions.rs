use super::*;

impl crate::DefineUnionNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<UnionDeclaration> {
        let mut errors = vec![];
        // let terms = self.function_body.build(ctx).recover(&mut errors)?;
        let annotations = self.annotation_head.annotations(ctx).recover(&mut errors)?;
        Success {
            value: UnionDeclaration {
                annotations,
                name: self.identifier.build(ctx),
                layout: None,
                derive_traits: vec![],
                terms: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: errors,
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
