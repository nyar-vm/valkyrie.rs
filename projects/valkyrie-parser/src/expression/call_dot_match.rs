use super::*;
impl crate::DotMatchCallNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<MatchCallNode> {
        let monadic = self.op_and_then.is_some();
        Ok(MatchCallNode {
            monadic,
            base: Default::default(),
            kind: self.kw_match.build(),
            patterns: PatternBlock { branches: vec![], span: Default::default() },
            span: self.span.clone(),
        })
    }
}
