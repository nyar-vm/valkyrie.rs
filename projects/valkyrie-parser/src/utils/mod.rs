use crate::{
    helpers::ProgramState, AnnotationTermMixNode, AnnotationTermNode, IfGuardNode, MatchTermsNode, ModifierAheadNode,
    TupleTermsNode, TypeHintNode,
};
use nyar_error::Result;
use valkyrie_ast::{ArgumentsList, AttributeList, ExpressionKind, ModifierList, PatternBranch};

pub(crate) fn build_arguments(this: &Option<TupleTermsNode>, ctx: &mut ProgramState) -> Result<ArgumentsList> {
    match this {
        Some(s) => s.build(ctx),
        None => Ok(ArgumentsList { terms: vec![] }),
    }
}
pub(crate) fn build_if_guard(this: &Option<IfGuardNode>, ctx: &mut ProgramState) -> Option<ExpressionKind> {
    match this.as_ref()?.build(ctx) {
        Ok(o) => Some(o),
        Err(e) => {
            ctx.add_error(e);
            None
        }
    }
}
pub(crate) fn build_type_hint(this: &Option<TypeHintNode>, ctx: &mut ProgramState) -> Option<ExpressionKind> {
    match this.as_ref()?.build(ctx) {
        Ok(o) => Some(o),
        Err(e) => {
            ctx.add_error(e);
            None
        }
    }
}

pub(crate) fn build_modifier_ahead(this: &[ModifierAheadNode], ctx: &mut ProgramState) -> ModifierList {
    ModifierList { terms: this.iter().map(|s| s.build(ctx)).collect() }
}

pub(crate) fn build_annotation_terms(this: &[AnnotationTermNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut terms = vec![];
    for term in this {
        match term.build(ctx) {
            Ok(o) => terms.push(o),
            Err(e) => ctx.add_error(e),
        }
    }
    Ok(AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() })
}

pub(crate) fn build_annotation_terms_mix(this: &[AnnotationTermMixNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut terms = vec![];
    for term in this {
        match term.build(ctx) {
            Ok(o) => terms.push(o),
            Err(e) => ctx.add_error(e),
        }
    }
    Ok(AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() })
}
