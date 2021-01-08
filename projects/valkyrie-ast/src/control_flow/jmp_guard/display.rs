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

impl PrettyPrint for GuardPattern {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Expression(node) => node.pretty(theme),
            Self::List(node) => node.pretty(theme),
            Self::Dict(node) => node.pretty(theme),
        }
    }
}
