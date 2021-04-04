use super::*;
use valkyrie_ast::TryStatement;

impl crate::TryStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<TryStatement> {
        Success {
            value: TryStatement { handler: None, body: Default::default(), span: self.span.clone() },
            diagnostics: vec![],
        }
    }
}
