use super::*;
use crate::PrettyTree;

impl PrettyPrint for IdentifierNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text(self.name.to_string())
    }
}

impl PrettyPrint for NamePathNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.intersperse(self.names.iter().map(|id| id.build(allocator)), allocator.text("∷"))
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

impl PrettyPrint for MacroPathNode {
    // fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    //     Display::fmt(&self.path, f)?;
    //     for item in &self.names {
    //         f.write_str(".")?;
    //         Display::fmt(item, f)?;
    //     }
    //     Ok(())
    // }

    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}
