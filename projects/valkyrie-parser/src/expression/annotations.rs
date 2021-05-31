use super::*;
use crate::{
    utils::{build_annotation_terms, build_annotation_terms_mix},
    AnnotationTermMixNode, AnnotationTermNode, AttributeItemNode, ClassBlockNode,
};
use nyar_error::Result;
// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Result<AnnotationNode> {
        let attributes = build_annotation_terms_mix(&self.annotation_term_mix, ctx)?;
        let modifiers = ModifierList { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() };
        Ok(AnnotationNode { documents: DocumentationList { terms: vec![] }, attributes, modifiers })
    }
}

impl crate::AnnotationHeadNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Result<AnnotationNode> {
        let attributes = build_annotation_terms(&self.annotation_term, ctx)?;
        let modifiers = ModifierList { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() };
        Ok(AnnotationNode { documents: DocumentationList { terms: vec![] }, attributes, modifiers })
    }
}

impl AttributeItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<AttributeTerm> {
        let path = self.namepath.build(ctx);
        // let domain = match &self.class_block {
        //     Some(s) => {
        //         Some(s.build(ctx)?)
        //     }
        //     None => {None}
        // };
        Ok(AttributeTerm { kind: Default::default(), path, variant: vec![], arguments: Default::default(), domain: None })
    }
}
impl AnnotationTermNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<AttributeList> {
        let mut terms = vec![];
        match self {
            Self::AttributeCall(v) => v.attribute_item.build(ctx)?,
            Self::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut ctx.errors)
                }
            }
        }
        Ok(AttributeList { terms })
    }
}

impl AnnotationTermMixNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<AttributeList> {
        let mut terms = vec![];
        match self {
            Self::AttributeCall(v) => v.attribute_item.build(ctx)?,
            Self::ProceduralCall(v) => v.attribute_item.build(ctx)?,
            Self::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut ctx.errors)
                }
            }
        }
        Ok(AttributeList { terms })
    }
}
