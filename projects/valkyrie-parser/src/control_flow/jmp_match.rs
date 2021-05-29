use super::*;
use nyar_error::NyarError;
use valkyrie_ast::{PatternBranch, PatternCaseNode, PatternCondition, PatternStatements};

impl crate::MatchExpressionNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<MatchStatement> {
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
    fn get_bind(&self, ctx: &mut ProgramState) -> Option<IdentifierNode> {
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

impl crate::MatchTermsNode {
    pub fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<PatternBranch>, es: &mut Vec<NyarError>) {
        match self {
            Self::MatchCase(v) => v.build(ctx).append(ts, es),
            Self::MatchElse(v) => v.build(ctx).append(ts, es),
            Self::MatchType(v) => v.build(ctx).append(ts, es),
            Self::MatchWhen(v) => v.build(ctx).append(ts, es),
            Self::Comma(_) => {}
        }
    }
}

impl crate::MatchCaseNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternBranch> {
        let mut diagnostics = vec![];
        Success {
            value: PatternBranch {
                condition: PatternCondition::Case(PatternCaseNode {
                    pattern: Default::default(),
                    guard: None,
                    span: Default::default(),
                }),
                statements: PatternStatements { terms: vec![] },
                span: self.span.clone(),
            },
            diagnostics,
        }
    }
}

impl crate::MatchTypeNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternBranch> {
        let mut diagnostics = vec![];
        Success {
            value: PatternBranch {
                condition: PatternCondition::Case(PatternCaseNode {
                    pattern: Default::default(),
                    guard: None,
                    span: Default::default(),
                }),
                statements: PatternStatements { terms: vec![] },
                span: self.span.clone(),
            },
            diagnostics,
        }
    }
}

impl crate::MatchWhenNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternBranch> {
        let mut diagnostics = vec![];
        Success {
            value: PatternBranch {
                condition: PatternCondition::Case(PatternCaseNode {
                    pattern: Default::default(),
                    guard: None,
                    span: Default::default(),
                }),
                statements: PatternStatements { terms: vec![] },
                span: self.span.clone(),
            },
            diagnostics,
        }
    }
}

impl crate::MatchElseNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Validation<PatternBranch> {
        let mut diagnostics = vec![];
        Success {
            value: PatternBranch {
                condition: PatternCondition::Case(PatternCaseNode {
                    pattern: Default::default(),
                    guard: None,
                    span: Default::default(),
                }),
                statements: PatternStatements { terms: vec![] },
                span: self.span.clone(),
            },
            diagnostics,
        }
    }
}
