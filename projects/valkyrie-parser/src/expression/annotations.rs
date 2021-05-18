use super::*;

// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Validation<AnnotationNode> {
        let mut errors = vec![];
        Success {
            value: AnnotationNode {
                documents: DocumentationList { terms: vec![] },
                attributes: Default::default(),
                modifiers: ModifierList { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() },
            },
            diagnostics: errors,
        }
    }
}

impl crate::AnnotationHeadNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Validation<AnnotationNode> {
        let mut errors = vec![];
        Success {
            value: AnnotationNode {
                documents: DocumentationList { terms: vec![] },
                attributes: Default::default(),
                modifiers: ModifierList { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() },
            },
            diagnostics: errors,
        }
    }
}
