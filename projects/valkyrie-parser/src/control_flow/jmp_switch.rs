use super::*;

impl crate::SwitchStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<SwitchStatement> {
        Ok(SwitchStatement { patterns: self.match_block.build(ctx), span: self.span.clone() })
    }
}
