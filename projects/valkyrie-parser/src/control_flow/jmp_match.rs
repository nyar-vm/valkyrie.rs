use super::*;

impl crate::MatchExpressionNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<MatchStatement> {
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

impl crate::KwMatchNode {
    pub(crate) fn build(&self) -> MatchKind {
        match self {
            Self::Match => MatchKind::Typing,
            Self::Catch => MatchKind::Effect,
        }
    }
}

impl crate::MatchTermsNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState, ts: &mut Vec<PatternBranch>) -> Result<()> {
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
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Case(self.build_node(ctx)?),
            statements: statements(&self.match_statement, ctx),
            span: self.span.clone(),
        })
    }
    fn build_node(&self, ctx: &mut ProgramState) -> Result<PatternCaseNode> {
        Ok(PatternCaseNode { pattern: Default::default(), guard: None, span: self.span.clone() })
    }
}

impl crate::MatchTypeNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Type(self.build_node(ctx)?),
            statements: statements(&self.match_statement, ctx),
            span: self.span.clone(),
        })
    }
    fn build_node(&self, ctx: &mut ProgramState) -> Result<PatternTypeNode> {
        Ok(PatternTypeNode { pattern: Default::default(), guard: None, span: self.span.clone() })
    }
}

impl crate::MatchWhenNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::When(self.build_node(ctx)?),
            statements: statements(&self.match_statement, ctx),
            span: self.span.clone(),
        })
    }
    fn build_node(&self, ctx: &mut ProgramState) -> Result<PatternWhenNode> {
        Ok(PatternWhenNode { guard: Default::default(), span: self.span.clone() })
    }
}

impl crate::MatchElseNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<PatternBranch> {
        Ok(PatternBranch {
            condition: PatternCondition::Else,
            statements: statements(&self.match_statement, ctx),
            span: self.span.clone(),
        })
    }
}

fn statements(statements: &[crate::MatchStatementNode], ctx: &mut ProgramState) -> PatternStatements {
    let mut list = PatternStatements::new(statements.len());
    for term in statements {
        match term.main_statement.build(ctx) {
            Ok(o) => list.terms.extend(o),
            Err(e) => ctx.add_error(e),
        }
    }
    list
}
