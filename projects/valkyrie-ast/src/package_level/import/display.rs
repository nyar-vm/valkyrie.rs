use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatementNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(3);
        items.push(allocator.keyword("using"));
        match &self.term {
            ImportTermNode::Alias(v) => {
                items.push(allocator.space());
                items.push(v.build(allocator));
            }
            ImportTermNode::Group(v) => {
                items.push(allocator.space());
                items.push(v.build(allocator));
            }
        }
        allocator.concat(items)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Alias(node) => node.build(allocator),
            Self::Group(node) => node.build(allocator),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportGroupNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(5);
        items.push(self.path.build(allocator));
        if !self.group.is_empty() {
            let bracket = KAndRBracket::curly_braces();
            items.push(bracket.build(&self.group, allocator, allocator.text(", "), allocator.hardline()))
        }
        allocator.concat(items)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportAliasNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(5);
        items.push(self.path.build(allocator));
        items.push(allocator.space());
        items.push(allocator.keyword("as"));
        items.push(allocator.space());
        items.push(self.alias.build(allocator));
        allocator.concat(items)
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
