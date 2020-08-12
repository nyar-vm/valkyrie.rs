use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatementNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(3);
        items.push(allocator.keyword("import"));
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

/// `K & R` style brackets
///
/// ```vk
/// a {}
/// ```
///
/// ```
/// a {}
/// ```
pub struct KAndRBracket {
    pub head_space: bool,
    pub bracket_l: &'static str,
    pub bracket_r: &'static str,
}

impl KAndRBracket {
    pub fn curly_braces() -> Self {
        Self { head_space: true, bracket_l: "{", bracket_r: "}" }
    }

    pub fn build<'a, I>(
        &self,
        items: &[I],
        allocator: &'a PrettyProvider<'a>,
        inline_join: PrettyTree<'a>,
        block_join: PrettyTree<'a>,
    ) -> PrettyTree<'a>
    where
        I: PrettyPrint,
    {
        let mut output = Vec::with_capacity(5);
        if self.head_space {
            output.push(allocator.space());
        }
        output.push(allocator.keyword(self.bracket_l));
        // inline
        let mut inline = Vec::with_capacity(output.len() + 1);
        inline.push(allocator.space());
        inline.push(allocator.intersperse(items, inline_join));
        inline.push(allocator.space());
        let inline = allocator.concat(inline);
        // block
        let mut block = Vec::with_capacity(output.len() + 1);
        block.push(allocator.hardline());
        block.push(allocator.intersperse(items, block_join).indent(4));
        block.push(allocator.hardline());
        let block = allocator.concat(block);
        //
        output.push(block.flat_alt(inline));
        output.push(allocator.keyword(self.bracket_r));
        allocator.concat(output)
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
