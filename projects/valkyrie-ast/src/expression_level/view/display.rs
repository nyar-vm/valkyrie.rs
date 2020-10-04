use super::*;

impl PrettyPrint for SubscriptNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = if self.index0 { "⁅" } else { "[" };
        let terms = theme.join(&self.terms, ", ");
        let rhs = if self.index0 { "⁆" } else { "]" };
        PrettyTree::StaticText(lhs).append(terms).append(rhs)
    }
}

impl PrettyPrint for SubscriptTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            SubscriptTermNode::Index(v) => v.pretty(theme),
            SubscriptTermNode::Slice(v) => v.pretty(theme),
        }
    }
}

impl PrettyPrint for SubscriptSliceNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let lhs = match &self.start {
            Some(s) => s.pretty(theme).append(":"),
            None => ":".into(),
        };
        let middle = match &self.end {
            Some(e) => PrettyTree::text(":").append(e.pretty(theme)),
            None => " :".into(),
        };
        match &self.step {
            Some(s) => lhs.append(middle).append(s.pretty(theme)),
            None => lhs.append(middle),
        }
    }
}
