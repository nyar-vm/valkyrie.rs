use super::*;

impl PrettyPrint for GuardStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("guard");
        terms += " ";
        terms += self.condition.pretty(theme);
        terms += " ";
        terms += theme.keyword("else");
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
        todo!()
    }
}
