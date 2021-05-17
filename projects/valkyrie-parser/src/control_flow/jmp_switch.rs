use super::*;
use valkyrie_ast::SwitchStatement;

impl crate::SwitchStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<SwitchStatement> {
        Success {
            value: SwitchStatement {
                patterns: PatternBlock { branches: vec![], span: Default::default() },
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
