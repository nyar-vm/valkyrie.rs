use super::*;

impl crate::MatchExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<MatchStatement> {
        Success {
            value: MatchStatement {
                kind: self.get_kind(),
                bind: self.get_bind(ctx),
                main: Default::default(),
                patterns: PatternBlock { branches: vec![], span: Default::default() },
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
    fn get_bind(&self, ctx: &ProgramContext) -> Option<IdentifierNode> {
        Some(self.identifier.as_ref()?.build(ctx))
    }
    fn get_kind(&self) -> MatchKind {
        match self.kw_match {
            KwMatchNode::Match => MatchKind::Typing,
            KwMatchNode::Catch => MatchKind::Effect,
        }
    }
}
