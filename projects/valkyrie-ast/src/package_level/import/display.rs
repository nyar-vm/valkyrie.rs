use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = Vec::with_capacity(3);
        items.push(theme.keyword("using"));
        match &self.term {
            ImportTermNode::Alias(v) => {
                items.push(theme.space());
                items.push(v.build(theme));
            }
            ImportTermNode::Group(v) => {
                items.push(theme.space());
                items.push(v.build(theme));
            }
        }
        theme.concat(items)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Alias(node) => node.build(theme),
            Self::Group(node) => node.build(theme),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportGroupNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = Vec::with_capacity(5);
        items.push(self.path.build(theme));
        if !self.group.is_empty() {
            let bracket = KAndRBracket::curly_braces();
            items.push(bracket.build(&self.group, theme, theme.text(", "), theme.hardline()))
        }
        theme.concat(items)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportAliasNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = Vec::with_capacity(5);
        items.push(self.path.build(theme));
        items.push(theme.space());
        items.push(theme.keyword("as"));
        items.push(theme.space());
        items.push(self.alias.build(theme));
        theme.concat(items)
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
