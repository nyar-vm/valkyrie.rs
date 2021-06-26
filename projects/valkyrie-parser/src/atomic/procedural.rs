use super::*;

impl crate::ProceduralCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ProceduralNode> {
        let attribute = self.attribute_item.build(ctx);
        Ok(ProceduralNode {
            kind: attribute.kind,
            path: attribute.path,
            arguments: attribute.arguments,
            domain: attribute.domain,
            span: self.span.clone(),
        })
    }
}
