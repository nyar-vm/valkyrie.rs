use super::*;
use crate::utils::build_type_hint;

impl crate::ObjectStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ConstructObjectNode> {
        Ok(ConstructObjectNode { base_classes: None, bounds: build_type_hint(&self.type_hint, ctx), span: self.span.clone() })
    }
}
