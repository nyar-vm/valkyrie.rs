use crate::{
    helpers::ProgramState, AnnotationTermMixNode, AnnotationTermNode, DefineConstraintNode, IfGuardNode, ModifierAheadNode,
    ParameterDefaultNode, TypeHintNode,
};
use nyar_error::Result;
use valkyrie_ast::{AttributeList, ConstraintDeclaration, ExpressionKind, ModifierList};

pub(crate) fn build_constraint(this: &Option<DefineConstraintNode>, ctx: &mut ProgramState) -> ConstraintDeclaration {
    match this {
        Some(s) => s.build(ctx),
        None => ConstraintDeclaration::default(),
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
pub(crate) fn build_parameter_default(this: &Option<ParameterDefaultNode>, ctx: &mut ProgramState) -> Option<ExpressionKind> {
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

pub(crate) fn build_annotation_terms(this: &[AnnotationTermNode], ctx: &mut ProgramState) -> AttributeList {
    let mut terms = Vec::with_capacity(this.len());
    for term in this {
        terms.push(term.build(ctx))
    }
    AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() }
}

pub(crate) fn build_annotation_terms_mix(this: &[AnnotationTermMixNode], ctx: &mut ProgramState) -> Result<AttributeList> {
    let mut terms = vec![];
    for term in this {
        terms.push(term.build(ctx))
    }
    Ok(AttributeList { terms: terms.into_iter().map(|v| v.terms).flatten().collect() })
}
