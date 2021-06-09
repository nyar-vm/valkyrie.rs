use super::*;
use crate::utils::build_match_terms;

impl crate::SwitchStatementNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<SwitchStatement> {
        let patterns = build_match_terms(&self.match_terms, ctx);
        Ok(SwitchStatement { patterns, span: self.span.clone() })
    }
}
