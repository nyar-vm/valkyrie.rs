use super::*;

impl crate::SwitchStatementNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<SwitchStatement> {
        let mut diagnostics = vec![];
        let mut patterns = vec![];
        for x in &self.match_terms {
            x.build(ctx, &mut patterns, &mut diagnostics)
        }
        Success { value: SwitchStatement { patterns, span: self.span.clone() }, diagnostics }
    }
}
