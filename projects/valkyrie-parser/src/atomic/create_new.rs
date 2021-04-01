use super::*;

impl crate::NewStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ConstructNewNode> {
        Success {
            value: ConstructNewNode {
                modifiers: vec![],
                namepath: NamePathNode { names: vec![] },
                generic: Default::default(),
                arguments: Default::default(),
                body: Default::default(),
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
