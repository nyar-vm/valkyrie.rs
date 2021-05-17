use super::*;

impl crate::DefineLambdaNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ConstructObjectNode> {
        Success { value: ConstructObjectNode { base_classes: None, span: self.span.clone() }, diagnostics: vec![] }
    }
}
