use super::*;

impl PrettyPrint for GuardStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms.push(theme.keyword("guard"));
        terms.push(theme.space());
        terms.push(self.condition.build(theme));
        terms.push(theme.space());
        terms.push(theme.keyword("else"));
        terms.push(self.body.build(theme));
        terms.into()
    }
}

impl PrettyPrint for GuardPattern {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Case(e) => e.pretty(theme),
            Self::Inline(e) => e.pretty(theme),
        }
    }
}
