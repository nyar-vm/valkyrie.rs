use super::*;
use crate::{PrettyPrint, PrettyProvider, PrettyTree};

impl PrettyPrint for ImportStatementNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "import ")?;
    //     Display::fmt(&self.r#type, f)?;
    //
    //     Ok(())
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for ImportStatementType {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Alias(node) => node.pretty(allocator),
            Self::Group(node) => node.pretty(allocator),
            Self::String(node) => node.pretty(allocator),
        }
    }
}

impl PrettyPrint for ImportTermNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Alias(node) => node.pretty(allocator),
            Self::Group(node) => node.pretty(allocator),
        }
    }
}

impl PrettyPrint for ImportGroupNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} {{ {} }}", self.path, self.group.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
impl PrettyPrint for ImportAliasNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     write!(f, "{} as {}", self.path, self.alias)
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
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
