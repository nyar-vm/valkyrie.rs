use super::*;
use crate::utils::build_annotation_terms;
use nyar_error::Validate;

impl crate::DefineLambdaNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<LambdaNode> {
        let attributes = build_annotation_terms(&self.annotation_term, ctx)?;
        let generic = self.function_middle.generics(ctx)?;
        let parameters = self.function_middle.parameters(ctx)?;
        let returns = self.function_middle.returns(ctx)?;
        let body = self.continuation.build(ctx)?;
        Ok(LambdaNode { annotations: attributes.into(), generics: generic, parameters, returns, body, span: self.span.clone() })
    }
}
