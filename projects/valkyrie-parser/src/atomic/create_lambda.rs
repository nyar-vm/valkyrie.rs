use super::*;

impl crate::DefineLambdaNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ConstructObjectNode> {
        Success { value: ConstructObjectNode { base_classes: None, span: self.span.clone() }, diagnostics: vec![] }
    }
}
