use super::*;
impl crate::DotClosureCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<ClosureCallNode> {
        let monadic = self.op_and_then.is_some();
        let trailing = self.continuation.build(ctx)?;
        Success {
            value: ClosureCallNode { monadic, base: Default::default(), trailing, span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
