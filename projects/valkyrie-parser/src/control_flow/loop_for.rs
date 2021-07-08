use super::*;

impl crate::ForStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ForLoop> {
        Ok(ForLoop {
            pattern: self.let_pattern.build(ctx)?,
            iterator: Default::default(),
            condition: self.if_guard.build(ctx),
            label: None,
            body: self.continuation.build(ctx),
            span: self.span.clone(),
        })
    }
}
