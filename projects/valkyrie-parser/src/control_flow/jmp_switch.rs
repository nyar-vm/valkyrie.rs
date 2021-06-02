use super::*;

impl crate::SwitchStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<SwitchStatement> {
        let mut patterns = vec![];
        for x in &self.match_terms {
            x.build(ctx, &mut patterns)?
        }
        Ok(SwitchStatement { patterns, span: self.span.clone() })
    }
}
