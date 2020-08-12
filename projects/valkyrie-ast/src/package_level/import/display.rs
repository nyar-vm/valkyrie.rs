use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatementNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "import ")?;
    //     Display::fmt(&self.r#type, f)?;
    //
    //     Ok(())
    // }

    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Item(node) => node.build(allocator),
            Self::Alias(node) => node.build(allocator),
            Self::Group(node) => node.build(allocator),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportGroupNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} {{ {} }}", self.path, self.group.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    // }

    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportAliasNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} as {}", self.path, self.alias)
    // }

    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
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
