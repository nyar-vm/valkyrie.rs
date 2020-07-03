use super::*;

impl Display for ImportStatementNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "import ")?;
        match &self.head {
            ImportStatementType::Nothing => {}
            ImportStatementType::Symbol(node) => Display::fmt(node, f)?,
            ImportStatementType::String(node) => Display::fmt(node, f)?,
        }
        if !self.group.is_empty() {
            write!(f, " {{ {} }}", self.group.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))?;
        }
        Ok(())
    }
}

impl Display for ImportTermNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alias(node) => Display::fmt(node, f),
            Self::Group(node) => Display::fmt(node, f),
        }
    }
}

impl Display for ImportGroupNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {{ {} }}", self.path, self.group.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}
impl Display for ImportAliasNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} as {}", self.path, self.alias)
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
