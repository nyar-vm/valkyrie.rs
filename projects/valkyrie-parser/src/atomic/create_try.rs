use super::*;

impl crate::TryStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<TryStatement> {
        let body = self.continuation.build(ctx)?;
        let handler = match &self.type_expression {
            Some(s) => Some(s.build(ctx)?),
            None => None,
        };
        Ok(TryStatement { handler, body, span: self.span.clone() })
    }
}
