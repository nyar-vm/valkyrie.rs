use super::*;

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.name)
    }
}

impl IndentDisplay for IdentifierNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        f.write_str(&self.name)
    }
}

impl IndentDisplay for NamePathNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        for (idx, item) in self.names.iter().enumerate() {
            if idx != 0 {
                f.write_str("∷")?;
            }
            item.indent_fmt(f)?;
        }
        Ok(())
    }
}

impl Display for NamePathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}

impl Display for MacroKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MacroKind::Normal => f.write_str("@"),
            MacroKind::Environment => f.write_str("@@"),
            MacroKind::Dict => f.write_str("@!"),
        }
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
