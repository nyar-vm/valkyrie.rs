use super::*;

impl PrettyPrint for SubscriptNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = theme.text(if self.index0 { "⁅" } else { "[" });
        let terms = theme.join(&self.terms, ", ");
        let rhs = theme.text(if self.index0 { "⁆" } else { "]" });
        lhs.append(terms).append(rhs)
    }
}

impl PrettyPrint for SubscriptTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            SubscriptTermNode::Index(v) => v.build(theme),
            SubscriptTermNode::Slice(v) => v.build(theme),
        }
    }
}

impl PrettyPrint for SubscriptSliceNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = match &self.start {
            Some(s) => s.build(theme).append(theme.text(":")),
            None => theme.text(":"),
        };
        let middle = match &self.end {
            Some(e) => theme.text(":").append(e.build(theme)),
            None => theme.text(" :"),
        };
        match &self.step {
            Some(s) => lhs.append(middle).append(s.build(theme)),
            None => lhs.append(middle),
        }
    }
}
