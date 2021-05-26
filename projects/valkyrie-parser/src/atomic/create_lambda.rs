use super::*;
use nyar_error::Validate;

impl crate::DefineLambdaNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<LambdaNode> {
        let mut errors = vec![];
        let body = self.continuation.build(ctx).recover(&mut errors)?;
        let returns = self.function_middle.returns(ctx).recover(&mut errors)?;
        let parameters = self.function_middle.parameters(ctx).recover(&mut errors)?;

        Success {
            value: LambdaNode {
                annotations: Default::default(),
                generic: None,
                parameters,
                returns,
                body,
                span: self.span.clone(),
            },
            diagnostics: errors,
        }
    }
}
