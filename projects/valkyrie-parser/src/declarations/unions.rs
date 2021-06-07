use super::*;
use crate::{utils::build_annotation_terms, UnionTermNode};

impl crate::DefineUnionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<UnionDeclaration> {
        // let terms = self.function_body.build(ctx).recover(&mut errors)?;
        let annotations = self.annotation_head.annotations(ctx)?;
        Ok(UnionDeclaration {
            annotations,
            name: self.identifier.build(ctx),
            layout: None,
            derive_traits: vec![],
            terms: self.terms(ctx),
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
            Self::ProceduralCall(v) => v.build(ctx)?.into(),
            Self::DefineVariant(v) => v.build(ctx)?.into(),
            Self::DefineMethod(v) => v.build(ctx)?.into(),
            Self::EosFree(_) => return Ok(None),
        };
        Ok(Some(value))
    }
}
impl crate::DefineVariantNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<VariantDeclaration> {
        let name = self.identifier.build(ctx);
        let annotations = build_annotation_terms(&self.annotation_term, ctx)?.into();
        Ok(VariantDeclaration { name, annotations, body: self.domain(ctx), span: self.span.clone() })
    }
    fn domain(&self, ctx: &mut ProgramState) -> Vec<ClassTerm> {
        let body = self.class_block.as_ref()?;
        match body.build(ctx) {
            Ok(o) => o,
            Err(e) => {
                ctx.add_error(e);
                vec![]
            }
        }
    }
}
