use super::*;

impl crate::ProceduralCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<ProceduralNode> {
        Ok(ProceduralNode {
            kind: Default::default(),
            path: NamePathNode { names: vec![] },
            arguments: Default::default(),
            domain: None,
            span: self.span.clone(),
        })
    }
}
