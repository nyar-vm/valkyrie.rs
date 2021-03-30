use super::*;

impl crate::GenericCallNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<GenericCallNode> {
        Success {
            value: GenericCallNode {
                monadic: false,
                base: Default::default(),
                terms: vec![],
                associated: vec![],
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
