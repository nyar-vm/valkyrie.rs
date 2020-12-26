use super::*;
use lispify::{Lisp, Lispify};

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
        theme.annotation(self.as_str())
    }
}

impl PrettyPrint for AnnotationNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms += self.kind.pretty(theme);
        terms += self.term.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for AnnotationList {
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

impl PrettyPrint for AnnotationTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.path.pretty(theme);
        terms += self.arguments.pretty(theme);
        terms += self.collects.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for AnnotationPathNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.annotation(self.to_string())
    }
}

impl Lispify for ModifiersNode {
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
