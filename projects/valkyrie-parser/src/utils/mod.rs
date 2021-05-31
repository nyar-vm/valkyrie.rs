use crate::{helpers::ProgramState, AnnotationTermMixNode, AnnotationTermNode, TupleTermsNode};
use nyar_error::{Result, Success, Validation};
use valkyrie_ast::{ArgumentsList, AttributeList};

pub fn build_arguments(this: &Option<TupleTermsNode>, ctx: &mut ProgramState) -> Result<ArgumentsList> {
    match this {
        Some(s) => s.build(ctx),
        None => Ok(ArgumentsList { terms: vec![] }),
    }
}

pub fn build_annotation_terms(this: &[AnnotationTermNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut diagnostics = vec![];
    let mut terms = vec![];
    for term in this {
        term.build(ctx).append(&mut terms, &mut diagnostics)
    }
    Success { value: AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() }, diagnostics }
}

pub fn build_annotation_terms_mix(this: &[AnnotationTermMixNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut diagnostics = vec![];
    let mut terms = vec![];
    for term in this {
        term.build(ctx).append(&mut terms, &mut diagnostics)
    }
    Success { value: AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() }, diagnostics }
}
