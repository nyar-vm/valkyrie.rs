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
        }
    }
    pub(crate) fn domain(&self, ctx: &mut ProgramState) -> Option<DomainDeclaration> {
        Some(self.class_block.as_ref()?.build_domain(ctx))
    }
}
impl crate::AnnotationTermNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<AttributeList> {
        let mut list = AttributeList::new(1);
        match self {
            Self::AttributeCall(v) => list.terms.push(v.attribute_item.build(ctx)),
            Self::AttributeList(v) => {
                for term in &v.attribute_item {
                    match term.build(ctx) {
                        Ok(o) => list.terms.push(o),
                        Err(e) => ctx.add_error(e),
                    }
                }
            }
        }
        Ok(list)
    }
}

impl crate::AnnotationTermMixNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<AttributeList> {
        let mut list = AttributeList::new(1);
        match self {
            Self::AttributeCall(v) => match v.attribute_item.build(ctx) {
                Ok(o) => list.terms.push(o),
                Err(e) => ctx.add_error(e),
            },
            Self::ProceduralCall(v) => match v.attribute_item.build(ctx) {
                Ok(o) => list.terms.push(o),
                Err(e) => ctx.add_error(e),
            },
            Self::AttributeList(v) => {
                for x in &v.attribute_item {
                    match x.build(ctx) {
                        Ok(o) => list.terms.push(o),
                        Err(e) => ctx.add_error(e),
                    }
                }
            }
        }
        Ok(list)
    }
}
impl crate::ModifierAheadNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> IdentifierNode {
        self.identifier.build(ctx)
    }
}
