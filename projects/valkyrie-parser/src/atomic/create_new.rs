use super::*;

impl crate::NewStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ConstructNewNode> {
        Success {
            value: ConstructNewNode {
                modifiers: vec![],
                namepath: self.namepath.build(ctx),
                generic: Default::default(),
                arguments: Default::default(),
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
