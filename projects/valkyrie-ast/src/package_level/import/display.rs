use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(3);
        items.push(theme.keyword("using"));
        match &self.term {
            ImportTermNode::Alias(v) => {
                items.push(" ");
                items.push(v.pretty(theme));
            }
            ImportTermNode::Group(v) => {
                items.push(" ");
                items.push(v.pretty(theme));
            }
        }
        items.into()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Alias(node) => node.pretty(theme),
            Self::Group(node) => node.pretty(theme),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportGroupNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.path.pretty(theme));
        if !self.group.is_empty() {
            let bracket = KAndRBracket::curly_braces();
            items += bracket.build(&self.group, theme, ", ".into(), PrettyTree::Hardline)
        }
        items.into()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportAliasNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.path.pretty(theme));
        items.push(" ");
        items.push(theme.keyword("as"));
        items.push(" ");
        items.push(self.alias.pretty(theme));
        items.into()
    }
}
impl From<ImportAliasNode> for ImportTermNode {
    fn from(value: ImportAliasNode) -> Self {
        Self::Alias(Box::new(value))
    }
}

impl From<ImportGroupNode> for ImportTermNode {
    fn from(value: ImportGroupNode) -> Self {
        Self::Group(Box::new(value))
    }
}
