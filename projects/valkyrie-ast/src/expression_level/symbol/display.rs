use super::*;

impl PrettyPrint for IdentifierNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> RefDoc<'a, ColorSpec> {
        allocator.text(self.name.to_string()).into_doc()
    }
}

impl PrettyPrint for NamePathNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> RefDoc<'a, ColorSpec> {
        allocator.intersperse(self.names.iter().map(|id| id.pretty(allocator)), allocator.text("∷")).into_doc()
    }
}

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Display for NamePathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.pretty_string(9999))
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

impl Display for MacroKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for MacroPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.path, f)?;
        for item in &self.names {
            f.write_str(".")?;
            Display::fmt(item, f)?;
        }
        Ok(())
    }
}
