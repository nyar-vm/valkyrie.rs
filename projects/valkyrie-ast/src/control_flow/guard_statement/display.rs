use super::*;

impl PrettyPrint for GuardStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("guard");
        terms += " ";
        terms += self.condition.pretty(theme);
        terms += " ";
        terms += theme.keyword("else");
        terms += self.body.pretty(theme);
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
