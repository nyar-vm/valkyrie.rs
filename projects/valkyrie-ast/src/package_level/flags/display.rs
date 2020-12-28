use super::*;

impl PrettyPrint for FlagsDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("flags");
        terms += " ";
        terms += self.name.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}

impl Lispify for FlagsDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Lisp::new(3 + self.body.terms.len());
        terms += Lisp::keyword("flags");
        terms += self.name.lispify();
        if let Some(s) = &self.layout {
            // terms += Lisp::keyword("layout") + s.lispify();
        }
        for term in &self.body.terms {
            terms += term.lispify();
        }
        terms
    }
}
