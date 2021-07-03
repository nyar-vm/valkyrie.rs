use super::*;

// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub(crate) fn annotations(&self, ctx: &mut ProgramState) -> Result<AnnotationNode> {
        let attributes = build_annotation_terms_mix(&self.annotation_term_mix, ctx)?;
        let modifiers = ModifierList { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() };
        Ok(AnnotationNode { documents: DocumentationList { terms: vec![] }, attributes, modifiers })
    }
}

impl crate::AnnotationHeadNode {
    pub(crate) fn annotations(&self, ctx: &mut ProgramState) -> AnnotationNode {
        let modifiers = ModifierList { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() };
        AnnotationNode {
            documents: DocumentationList { terms: vec![] },
            attributes: build_annotation_terms(&self.annotation_term, ctx),
            modifiers,
        }
    }
}

impl crate::AttributeItemNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> AttributeTerm {
        AttributeTerm {
            kind: Default::default(),
            path: self.namepath.build(ctx),
            variant: vec![],
            arguments: Default::default(),
            domain: self.domain(ctx),
            span: self.span.clone(),
        }
    }
    fn domain(&self, ctx: &mut ProgramState) -> Option<StatementBlock> {
        Some(self.continuation.as_ref()?.build(ctx))
    }
}
impl crate::AnnotationTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> AttributeList {
        let terms = match self {
            Self::AttributeCall(v) => vec![v.attribute_item.build(ctx)],
            Self::AttributeList(v) => v.attribute_item.iter().map(|v| v.build(ctx)).collect(),
        };
        AttributeList { terms }
    }
}

impl crate::AnnotationTermMixNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> AttributeList {
        let terms = match self {
            Self::AttributeCall(v) => vec![v.attribute_item.build(ctx)],
            Self::ProceduralCall(v) => vec![v.build(ctx).into()],
            Self::AttributeList(v) => v.attribute_item.iter().map(|v| v.build(ctx)).collect(),
        };
        AttributeList { terms }
    }
}

impl crate::ModifierAheadNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> IdentifierNode {
        self.identifier.build(ctx)
    }
}
