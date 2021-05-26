use super::*;
use crate::utils::build_annotation_terms;
use nyar_error::Validate;

impl crate::DefineLambdaNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<LambdaNode> {
        let mut diagnostics = vec![];
        let attributes = build_annotation_terms(&self.annotation_term, ctx).recover(&mut diagnostics)?;
        let generic = self.function_middle.generic(ctx).recover(&mut diagnostics)?;
        let parameters = self.function_middle.parameters(ctx).recover(&mut diagnostics)?;
        let returns = self.function_middle.returns(ctx).recover(&mut diagnostics)?;
        let body = self.continuation.build(ctx).recover(&mut diagnostics)?;
        Success {
            value: LambdaNode { annotations: attributes.into(), generic, parameters, returns, body, span: self.span.clone() },
            diagnostics,
        }
    }
}
