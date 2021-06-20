use super::*;
use crate::utils::{build_if_guard, build_type_hint};

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
            body: Default::default(),
            span: self.span.clone(),
        })
    }
}
