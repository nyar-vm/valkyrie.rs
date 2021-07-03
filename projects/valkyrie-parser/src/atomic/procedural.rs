use super::*;

impl crate::ProceduralCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> ProceduralNode {
        ProceduralNode {
            kind: Default::default(),
            path: NamePathNode { names: vec![] },
            arguments: Default::default(),
            domain: None,
            span: self.span.clone(),
        }
    }
}
