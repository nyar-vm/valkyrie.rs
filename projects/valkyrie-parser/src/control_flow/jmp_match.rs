use super::*;

impl crate::MatchExpressionNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<MatchStatement> {
        Success {
            value: MatchStatement {
                kind: self.kw_match.build(),
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
}

impl KwMatchNode {
    pub fn build(&self) -> MatchKind {
        match self {
            KwMatchNode::Match => MatchKind::Typing,
            KwMatchNode::Catch => MatchKind::Effect,
        }
    }
}
