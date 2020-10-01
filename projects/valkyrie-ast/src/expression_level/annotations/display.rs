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
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.metadata(self.as_str())
    }
}

impl PrettyPrint for AnnotationNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms.push(self.kind.build(theme));
        terms.push(self.term.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for AnnotationList {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.terms.len());
        terms.push(self.kind.build(theme));
        terms.push(theme.metadata("["));
        for term in &self.terms {
            terms.push(term.build(theme));
        }
        terms.push(theme.metadata("]"));
        theme.concat(terms)
    }
}

impl PrettyPrint for AnnotationTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(self.path.build(theme));
        terms.push(self.arguments.build(theme));
        terms.push(self.collects.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for AnnotationPathNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.metadata(self.to_string())
    }
}
