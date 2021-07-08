use super::*;

impl Debug for ImportStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("ImportStatement");
        if !self.annotation.is_empty() {
            w.field("annotation", &self.annotation);
        }
        w.field("kind", &self.kind).field("term", &self.term).finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(3);
        items.push(theme.keyword("using"));
        match &self.term {
            ImportTermNode::Alias(v) => {
                items.push(" ");
                items.push(v.pretty(theme));
            }
            ImportTermNode::Group(v) => {
                items.push(" ");
                items.push(v.pretty(theme));
            }
            ImportTermNode::All(_) => {}
        }
        items.into()
    }
}

impl Debug for ImportTermNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Group(node) => Debug::fmt(node, f),
            Self::All(node) => Debug::fmt(node, f),
            Self::Alias(node) => Debug::fmt(node, f),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Alias(node) => node.pretty(theme),
            Self::Group(node) => node.pretty(theme),
            Self::All(node) => node.pretty(theme),
        }
    }
}

impl Debug for ImportGroupNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("ImportGroupNode");
        if !self.path.is_empty() {
            w.field("path", &IdentifiersDisplay::new(&self.path));
        }
        w.field("item", &self.terms);
        w.finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportGroupNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.path.pretty(theme));
        if !self.terms.is_empty() {
            let bracket = KAndRBracket::curly_braces();
            items += bracket.build(&self.terms, theme, ", ".into(), PrettyTree::Hardline)
        }
        items.into()
    }
}
impl Debug for ImportAliasNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("ImportAliasNode");
        if !self.path.is_empty() {
            w.field("path", &IdentifiersDisplay::new(&self.path));
        }
        w.field("item", &WrapDisplay::new(&self.item));
        if let Some(alias) = &self.alias {
            w.field("alias", &WrapDisplay::new(alias));
        }
        w.finish()
    }
}

impl Display for ImportAliasItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Attribute(id) => write!(f, "#{id}"),
            Self::Procedural(id) => write!(f, "@{id}"),
            Self::Normal(id) => write!(f, "{id}"),
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ImportAliasNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(5);
        items.push(self.path.pretty(theme));
        items.push(" ");
        items.push(theme.keyword("as"));
        items.push(" ");
        items.push(self.alias.pretty(theme));
        items.into()
    }
}
impl Display for ImportResolvedItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for (index, name) in self.path.iter().enumerate() {
            if index != 0 {
                f.write_char('.')?
            }
            f.write_str(name.as_ref())?
        }
        Display::fmt(&self.kind, f)
    }
}

impl Display for ImportResolvedKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty => {}
            Self::All => f.write_str(".*")?,
            Self::This => {}
            Self::Alias { item, name } => {
                write!(f, ".{}", item)?;
                if let Some(name) = name {
                    write!(f, " as {}", name)?;
                }
            }
        }
        Ok(())
    }
}
