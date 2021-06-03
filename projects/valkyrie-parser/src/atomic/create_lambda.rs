use super::*;

impl crate::DefineLambdaNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<LambdaNode> {
        let attributes = build_annotation_terms(&self.annotation_term, ctx)?;
        let generics = self.function_middle.generics(ctx)?;
        let parameters = self.function_middle.parameters(ctx)?;
        let returns = self.function_middle.returns(ctx)?;
        let body = self.continuation.build(ctx)?;
        Ok(LambdaNode { annotations: attributes.into(), generics, parameters, returns, body, span: self.span.clone() })
    }
}
