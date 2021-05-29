use super::*;

impl crate::ObjectStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ConstructObjectNode> {
        let mut diagnostics = vec![];
        let bounds = match &self.type_hint {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success { value: ConstructObjectNode { base_classes: None, bounds, span: self.span.clone() }, diagnostics }
    }
}
