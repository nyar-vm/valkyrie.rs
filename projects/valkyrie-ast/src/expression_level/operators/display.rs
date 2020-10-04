use super::*;

impl PrettyPrint for OperatorNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.operator(self.kind.as_str())
    }
}

impl PrettyPrint for PrefixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.operator.pretty(theme).append(self.base.pretty(theme))
    }
}

impl PrettyPrint for InfixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.lhs.pretty(theme));
        items.push(" ");
        items.push(self.operator.pretty(theme));
        items.push(" ");
        items.push(self.rhs.pretty(theme));
        items.into()
    }
}

impl PrettyPrint for PostfixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.base.pretty(theme).append(self.operator.pretty(theme))
    }
}
