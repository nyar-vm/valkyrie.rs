use super::*;
use crate::utils::build_parameter_default;

impl crate::DefineVariableNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<VariableDeclaration> {
        Ok(VariableDeclaration {
            pattern: PatternNode::Tuple(Box::new(TuplePatternNode {
                bind: None,
                name: None,
                terms: vec![],
                span: Default::default(),
            })),
            type_hint: build_type_hint(&self.type_hint, ctx),
            body: build_parameter_default(&self.parameter_default, ctx),
            span: self.span.clone(),
        })
    }
}

impl crate::ParameterDefaultNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionKind> {
        self.main_expression.build(ctx)
    }
}
