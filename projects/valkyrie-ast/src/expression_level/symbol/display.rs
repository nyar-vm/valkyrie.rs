use super::*;
use crate::PrettyTree;

impl PrettyPrint for IdentifierNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text(self.name.to_string())
    }
}

impl PrettyPrint for NamePathNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.intersperse(self.names.iter().map(|id| id.pretty(allocator)), allocator.text("∷"))
    }
}

impl MacroKind {
    /// Returns the string representation of the macro kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            MacroKind::Normal => "@",
            MacroKind::Environment => "@@",
            MacroKind::NonCapture => "@!",
        }
    }
}

impl PrettyPrint for MacroKind {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     f.write_str(self.as_str())
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for MacroPathNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     Display::fmt(&self.path, f)?;
    //     for item in &self.names {
    //         f.write_str(".")?;
    //         Display::fmt(item, f)?;
    //     }
    //     Ok(())
    // }

    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
