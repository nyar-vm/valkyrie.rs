use super::*;

impl crate::DefineVariableNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<VariableDeclaration> {
        Ok(VariableDeclaration {
            pattern: PatternNode::Tuple(Box::new(TuplePatternNode {
                bind: None,
                name: None,
                terms: vec![],
                span: Default::default(),
            })),
            type_hint: None,
            body: Default::default(),
            span: self.span.clone(),
        })
    }
}
