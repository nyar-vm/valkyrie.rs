use super::*;

// static PREFIX: &'static str = r#"^(?x)(
//       [+\-±]
//     | [¬!~]
//     | [⅟√∛∜]
//     | [*]{1,3}
//     | [⁑⁂]
// )"#;

impl crate::AnnotationMixNode {
    pub fn modifiers(&self, ctx: &ProgramContext) -> ModifiersNode {
        ModifiersNode { terms: self.modifier_ahead.iter().map(|s| s.identifier.build(ctx)).collect() }
    }
    pub fn document(&self, _: &ProgramContext) -> DocumentationNode {
        DocumentationNode { documentation: "".to_string(), span: Default::default() }
    }
}

impl crate::AnnotationHeadNode {
    pub fn modifiers(&self, ctx: &ProgramContext) -> ModifiersNode {
        ModifiersNode { terms: self.modifier_call.iter().map(|s| s.identifier.build(ctx)).collect() }
    }
    pub fn document(&self, _: &ProgramContext) -> DocumentationNode {
        DocumentationNode { documentation: "".to_string(), span: Default::default() }
    }
}
