use super::*;
use crate::KwMatchNode;
use valkyrie_ast::{IdentifierNode, MatchKind, MatchStatement, PatternBlock};

impl crate::MatchStatementNode {
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
        Some(self.identifier?.build(ctx))
    }
    fn get_kind(&self) -> MatchKind {
        match self.kw_match {
            KwMatchNode::Until => MatchKind::Typing,
            KwMatchNode::While => MatchKind::Effect,
        }
    }
}
