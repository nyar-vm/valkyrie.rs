use super::*;

impl PrettyPrint for OperatorNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.operator(self.kind.as_str())
    }
}

impl PrettyPrint for PrefixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.operator.build(theme).append(self.base.build(theme))
    }
}

impl PrettyPrint for InfixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = Vec::with_capacity(5);
        items.push(self.lhs.build(theme));
        items.push(theme.space());
        items.push(self.operator.build(theme));
        items.push(theme.space());
        items.push(self.rhs.build(theme));
        theme.concat(items)
    }
}

impl PrettyPrint for PostfixNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.base.build(theme).append(self.operator.build(theme))
    }
}
