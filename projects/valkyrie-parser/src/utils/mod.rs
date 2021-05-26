use crate::{helpers::ProgramState, AnnotationTermMixNode, AnnotationTermNode, TupleTermsNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ArgumentsList, AttributeList};

pub fn build_arguments(this: &Option<TupleTermsNode>, ctx: &mut ProgramState) -> Validation<ArgumentsList> {
    match this {
        Some(s) => s.build(ctx),
        None => Success { value: ArgumentsList { terms: vec![] }, diagnostics: vec![] },
    }
}

pub fn build_annotation_terms(this: &[AnnotationTermNode], ctx: &mut ProgramState) -> Validation<AttributeList> {
    let mut diagnostics = vec![];
    let mut terms = vec![];
    for term in this {
        match term {
            AnnotationTermNode::AttributeCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            AnnotationTermNode::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut diagnostics)
                }
            }
        }
    }

    Success { value: AttributeList { terms }, diagnostics }
}

pub fn build_annotation_terms_mix(this: &[AnnotationTermMixNode], ctx: &mut ProgramState) -> Validation<AttributeList> {
    let mut diagnostics = vec![];
    let mut terms = vec![];
    for term in this {
        match term {
            AnnotationTermMixNode::AttributeCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            AnnotationTermMixNode::ProceduralCall(v) => v.attribute_item.build(ctx).append(&mut terms, &mut diagnostics),
            AnnotationTermMixNode::AttributeList(v) => {
                for x in &v.attribute_item {
                    x.build(ctx).append(&mut terms, &mut diagnostics)
                }
            }
        }
    }

    Success { value: AttributeList { terms }, diagnostics }
}
