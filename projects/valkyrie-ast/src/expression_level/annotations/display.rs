use super::*;

impl Display for AnnotationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for AnnotationPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.path, f)?;
        for name in &self.names {
            f.write_char('.')?;
            Display::fmt(name, f)?;
        }
        Ok(())
    }
}

impl PrettyPrint for AnnotationKind {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.metadata(self.as_str())
    }
}

impl PrettyPrint for AnnotationNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for AnnotationList {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(self.terms.len());
        terms.push(self.kind.build(allocator));
        terms.push(allocator.metadata("["));
        for term in &self.terms {
            terms.push(term.build(allocator));
        }
        terms.push(allocator.metadata("]"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for AnnotationTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.path.build(allocator));
        terms.push(self.arguments.build(allocator));
        terms.push(self.collects.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for AnnotationPathNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(self.names.len() + 1);
        terms.push(self.path.build(allocator));
        for item in &self.names {
            terms.push(allocator.metadata("."));
            terms.push(item.build(allocator));
        }
        allocator.concat(terms)
    }
}
