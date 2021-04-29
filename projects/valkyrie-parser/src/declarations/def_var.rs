use super::*;

impl crate::DefineVariableNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<VariableDeclaration> {
        Success {
            value: VariableDeclaration {
                pattern: PatternNode::Tuple(Box::new(TuplePatternNode {
                    bind: None,
                    name: None,
                    terms: vec![],
                    span: Default::default(),
                })),
                type_hint: None,
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
