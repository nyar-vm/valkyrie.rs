use super::*;

impl crate::NewStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ConstructNewNode> {
        let mut errors = vec![];
        // let body = self.mo.build(ctx).recover(&mut errors)?;
        // let returns = self.function_middle.returns(ctx).recover(&mut errors)?;
        Success {
            value: ConstructNewNode {
                modifiers: vec![],
                namepath: self.namepath.build(ctx),
                generic: Default::default(),
                arguments: Default::default(),
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: errors,
        }
    }
}
