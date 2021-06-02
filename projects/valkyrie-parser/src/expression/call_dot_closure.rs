use super::*;
impl crate::DotClosureCallNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ClosureCallNode> {
        let monadic = self.op_and_then.is_some();
        let trailing = self.continuation.build(ctx)?;
        Ok(ClosureCallNode { monadic, base: Default::default(), trailing, span: self.span.clone() })
    }
}
