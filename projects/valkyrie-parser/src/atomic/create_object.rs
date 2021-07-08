use super::*;

impl crate::ObjectStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ConstructObjectNode> {
        Ok(ConstructObjectNode { base_classes: None, bounds: self.type_hint.build(ctx), span: self.span.clone() })
    }
}
