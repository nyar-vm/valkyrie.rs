use super::*;
use crate::{
    utils::{build_annotation_terms, build_annotation_terms_mix},
    AnnotationTermMixNode, AnnotationTermNode, AttributeItemNode, ClassBlockNode,
};

// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Validation<AnnotationNode> {
        let mut diagnostics = vec![];
        let attributes = build_annotation_terms_mix(&self.annotation_term_mix, ctx).recover(&mut diagnostics)?;
        let modifiers = ModifierList { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() };
        Success { value: AnnotationNode { documents: DocumentationList { terms: vec![] }, attributes, modifiers }, diagnostics }
    }
}

impl crate::AnnotationHeadNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Validation<AnnotationNode> {
        let mut diagnostics = vec![];
        let attributes = build_annotation_terms(&self.annotation_term, ctx).recover(&mut diagnostics)?;
        let modifiers = ModifierList { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() };
        Success { value: AnnotationNode { documents: DocumentationList { terms: vec![] }, attributes, modifiers }, diagnostics }
    }
}

impl AttributeItemNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<AttributeTerm> {
        let mut diagnostics = vec![];
        let path = self.namepath.build(ctx);
        // let domain = match &self.class_block {
        //     Some(s) => {
        //         Some(s.build(ctx).recover(&mut diagnostics)?)
        //     }
        //     None => {None}
        // };
        Success {
            value: AttributeTerm {
                kind: Default::default(),
                path,
                variant: vec![],
                arguments: Default::default(),
                domain: None,
            },
            diagnostics,
        }
    }
}
impl AnnotationTermNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<AttributeList> {
        let mut diagnostics = vec![];
        let mut terms = vec![];
        match self {
            Self::AttributeCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            Self::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut diagnostics)
                }
            }
        }
        Success { value: AttributeList { terms }, diagnostics }
    }
}

impl AnnotationTermMixNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<AttributeList> {
        let mut diagnostics = vec![];
        let mut terms = vec![];
        match self {
            Self::AttributeCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            Self::ProceduralCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            Self::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut diagnostics)
                }
            }
        }
        Success { value: AttributeList { terms }, diagnostics }
    }
}
