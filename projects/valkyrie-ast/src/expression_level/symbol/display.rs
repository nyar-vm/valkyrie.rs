use super::*;

impl Display for IdentifierNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl Display for NamePathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut iter = self.names.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, "∷{}", item)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrint for IdentifierNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text(self.name.to_string())
    }
}

impl PrettyPrint for LambdaSlotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword(format!("${}", self.name))
    }
}

impl PrettyPrint for NamePathNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.join(&self.names, "∷")
    }
}
