use super::*;

impl crate::ObjectStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ConstructObjectNode> {
        let bounds = match &self.type_hint {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Ok(ConstructObjectNode { base_classes: None, bounds, span: self.span.clone() })
    }
}
