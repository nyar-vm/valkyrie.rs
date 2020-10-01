use super::*;

impl PrettyPrint for PatternBranch {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![self.condition.build(theme), self.statements.build(theme)])
    }
}

impl PrettyPrint for PatternCondition {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let item = match self {
            Self::Case(v) => v.build(theme),
            Self::When(v) => v.build(theme),
            Self::Type(v) => v.build(theme),
            Self::Else(v) => v.build(theme),
        };
        item.append(theme.text(":")).append(theme.hardline())
    }
}

impl PrettyPrint for PatternStatements {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.terms.len() * 2);
        let len = self.terms.len();
        for (idx, term) in self.terms.iter().enumerate() {
            terms.push(term.build(theme));
            if idx == len.saturating_sub(1) {
                terms.push(theme.text(","));
            }
        }
        theme.concat(terms).indent(4)
    }
}

impl PrettyPrint for ImplicitCaseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms.push(self.pattern.build(theme));
        terms.push(theme.keyword("≔"));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for PatternCaseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms.push(theme.keyword("case"));
        terms.push(theme.space());
        terms.push(self.pattern.build(theme));
        if let Some(guard) = &self.guard {
            terms.push(theme.keyword("when"));
            terms.push(guard.build(theme));
        }
        theme.concat(terms)
    }
}

impl PrettyPrint for PatternWhenNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![theme.keyword("when"), theme.space(), self.guard.build(theme)])
    }
}

impl PrettyPrint for PatternTypeNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.concat(vec![theme.keyword("type"), theme.space(), self.pattern.build(theme)])
    }
}

impl PrettyPrint for PatternElseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword("else")
    }
}
