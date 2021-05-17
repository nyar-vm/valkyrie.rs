use super::*;

impl Debug for AnnotationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.is_empty() {
            f.debug_struct("Empty").finish()
        }
        else {
            let mut w = f.debug_struct("Annotation");
            if !self.documents.is_empty() {
                w.field("documents", &self.documents);
            }
            if !self.attributes.is_empty() {
                w.field("attributes", &self.attributes);
            }
            if !self.modifiers.is_empty() {
                w.field("modifiers", &WrapDisplay::new(&self.modifiers));
            }
            w.finish()
        }
    }
}

impl Display for AttributeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for AttributeKind {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.annotation(self.as_str())
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for AttributeNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms += self.kind.pretty(theme);
        terms += self.term.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for AttributeList {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.terms.len());
        terms += self.kind.pretty(theme);
        terms += theme.annotation("[");
        for term in &self.terms {
            terms += term.pretty(theme);
        }
        terms += theme.annotation("]");
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for AttributeTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.path.pretty(theme);
        terms += self.arguments.pretty(theme);
        terms += self.domain.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for AttributeName {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.annotation(self.to_string())
    }
}

impl Display for ModifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.terms.iter()).finish()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ModifierList {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("modifiers");
        for modifier in &self.terms {
            lisp += modifier.lispify();
        }
        lisp
    }
}
