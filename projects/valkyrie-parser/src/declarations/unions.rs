use super::*;

impl crate::DefineUnionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<UnionDeclaration> {
        Ok(UnionDeclaration {
            annotations: self.annotation_head.annotations(ctx),
            name: self.identifier.build(ctx),
            inherits: vec![],
            implements: build_type_hint(&self.type_hint, ctx),
            body: self.terms(ctx),
            span: self.span.clone(),
        })
    }

    fn terms(&self, ctx: &mut ProgramState) -> Vec<UnionTerm> {
        let mut terms = Vec::with_capacity(self.union_term.len());
        for term in &self.union_term {
            match term.build(ctx) {
                Ok(o) => terms.extend(o),
                Err(e) => {
                    ctx.add_error(e);
                }
            }
        }
        terms
    }
}

impl crate::KwUnionNode {
    // pub(crate) fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
impl crate::UnionTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<Option<UnionTerm>> {
        let value = match self {
            Self::ProceduralCall(v) => v.build(ctx).into(),
            Self::DefineVariant(v) => v.build(ctx)?.into(),
            Self::DefineMethod(v) => v.build(ctx)?.into(),
            Self::EosFree(_) => return Ok(None),
        };
        Ok(Some(value))
    }
}
impl crate::DefineVariantNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<VariantDeclaration> {
        Ok(VariantDeclaration {
            name: self.identifier.build(ctx),
            annotations: build_annotation_terms(&self.annotation_term, ctx).into(),
            body: self.domain(ctx),
            span: self.span.clone(),
        })
    }
    fn domain(&self, ctx: &mut ProgramState) -> Vec<ClassTerm> {
        match &self.class_block {
            Some(body) => body.build(ctx),
            None => vec![],
        }
    }
}
