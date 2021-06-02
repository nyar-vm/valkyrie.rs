use super::*;

impl crate::DefineUnionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<UnionDeclaration> {
        // let terms = self.function_body.build(ctx).recover(&mut errors)?;
        let annotations = self.annotation_head.annotations(ctx)?;
        Ok(UnionDeclaration {
            annotations,
            name: self.identifier.build(ctx),
            layout: None,
            derive_traits: vec![],
            terms: Default::default(),
            span: self.span.clone(),
        })
    }
}

impl crate::KwUnionNode {
    // pub(crate) fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
