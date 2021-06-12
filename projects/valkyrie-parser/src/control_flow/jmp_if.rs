use super::*;
use valkyrie_ast::ExpressionKind;

impl crate::IfGuardNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<ExpressionKind> {
        self.inline_expression.build(ctx)
    }
}
