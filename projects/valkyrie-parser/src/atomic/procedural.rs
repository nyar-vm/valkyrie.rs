use super::*;

impl crate::ProceduralCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ProceduralNode> {
        let path = self.attribute_item.namepath.build(ctx);
        Ok(ProceduralNode {
            kind: Default::default(),
            path,
            arguments: Default::default(),
            domain: self.domain(ctx),
            span: self.span.clone(),
        })
    }
    fn domain(&self, ctx: &mut ProgramState) -> Option<DomainDeclaration> {
        match &self.attribute_item.class_block {
            Some(v) => match v.build_domain(ctx) {
                Ok(o) => Some(o),
                Err(e) => {
                    ctx.add_error(e);
                    None
                }
            },
            None => None,
        }
    }
}
