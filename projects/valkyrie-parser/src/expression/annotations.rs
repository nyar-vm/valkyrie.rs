use super::*;

// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub fn modifiers(&self, ctx: &mut ProgramState) -> ModifierList {
        ModifierList { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() }
    }
    pub fn document(&self, _: &mut ProgramState) -> DocumentationNode {
        DocumentationNode { documentation: "".to_string(), span: Default::default() }
    }
}

impl crate::AnnotationHeadNode {
    pub fn annotations(&self, ctx: &mut ProgramState) -> Validation<AnnotationNode> {
        let mut errors = vec![];
        Success {
            value: AnnotationNode {
                documents: DocumentationNode { documentation: "".to_string(), span: Default::default() },
                attributes: Default::default(),
                modifiers: ModifierList { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() },
            },
            diagnostics: errors,
        }
    }
}
