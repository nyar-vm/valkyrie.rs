use super::*;

impl crate::MatchExpressionNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<MatchStatement> {
        let mut patterns = vec![];
        for x in &self.match_terms {
            x.build(ctx, &mut patterns)?
        }
        self.inline_expression.build(ctx)?;

        Ok(MatchStatement {
            kind: self.kw_match.build(),
            bind: self.get_bind(ctx),
            main: Default::default(),
            patterns,
            span: self.span.clone(),
        })
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
    pub fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<PatternBranch>) -> Result<()> {
        let value = match self {
            Self::MatchCase(v) => v.build(ctx)?,
            Self::MatchElse(v) => v.build(ctx)?,
            Self::MatchType(v) => v.build(ctx)?,
            Self::MatchWhen(v) => v.build(ctx)?,
            Self::Comma(_) => return Ok(()),
        };
        ts.push(value);
        Ok(())
    }
}

impl crate::MatchCaseNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Case(PatternCaseNode {
                pattern: Default::default(),
                guard: None,
                span: Default::default(),
            }),
            statements: PatternStatements { terms: vec![] },
            span: self.span.clone(),
        })
    }
}

impl crate::MatchTypeNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Case(PatternCaseNode {
                pattern: Default::default(),
                guard: None,
                span: Default::default(),
            }),
            statements: PatternStatements { terms: vec![] },
            span: self.span.clone(),
        })
    }
}

impl crate::MatchWhenNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Case(PatternCaseNode {
                pattern: Default::default(),
                guard: None,
                span: Default::default(),
            }),
            statements: PatternStatements { terms: vec![] },
            span: self.span.clone(),
        })
    }
}

impl crate::MatchElseNode {
    pub fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        let mut terms = vec![];
        for x in &self.match_statement {
            x.main_statement.build(ctx, &mut terms)?
        }
        Ok(PatternBranch {
            condition: PatternCondition::Else,
            statements: PatternStatements { terms },
            span: self.span.clone(),
        })
    }
}
