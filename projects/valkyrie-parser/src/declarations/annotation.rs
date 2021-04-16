use super::*;

impl crate::AnnotationHeadNode {
    pub fn get_modifiers(&self, ctx: &ProgramContext) -> ModifiersNode {
        ModifiersNode { terms: self.modifier_call.iter().map(|v| v.identifier.build(ctx)).collect() }
    }
}
