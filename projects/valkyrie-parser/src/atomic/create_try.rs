use super::*;

impl crate::TryStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<TryStatement> {
        let mut errors = vec![];
        let body = self.continuation.build(ctx).recover(&mut errors)?;
        let handler = match &self.type_expression {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Success { value: TryStatement { handler, body, span: self.span.clone() }, diagnostics: errors }
    }
}
