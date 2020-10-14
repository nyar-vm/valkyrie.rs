use super::*;

impl PrettyPrint for GuardStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("guard");
        terms += " ";
        terms += self.condition.pretty(theme);
        terms += " ";
        terms += self.main_body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for GuardStatementBody {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Positive(node) => node.pretty(theme),
            Self::Negative(node) => node.pretty(theme),
        }
    }
}

impl PrettyPrint for GuardLetStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("guard let");
        terms += " ";
        terms += self.pattern.pretty(theme);
        terms += " ";
        terms += self.expression.pretty(theme);
        terms += " ";
        terms += self.main_body.pretty(theme);
        terms.into()
    }
}
