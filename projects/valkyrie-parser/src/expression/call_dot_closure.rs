use super::*;
impl crate::DotClosureCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ClosureCallNode> {
        let monadic = self.op_and_then.is_some();
        Success {
            value: ClosureCallNode {
                monadic,
                base: Default::default(),
                caller: ClosureCaller::Normal,
                trailing: None,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
