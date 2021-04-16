use super::*;
impl crate::DotClosureCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ClosureCallNode> {
        let monadic = self.op_and_then.is_some();
        Success {
            value: ClosureCallNode {
                monadic,
                base: Default::default(),
                trailing: StatementBlock { terms: vec![], span: Default::default() },
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
