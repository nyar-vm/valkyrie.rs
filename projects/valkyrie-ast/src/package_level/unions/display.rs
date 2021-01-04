use super::*;

impl PrettyPrint for UnionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("union");
        terms += " ";
        terms += self.name.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for UnionDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);
        lisp += Lisp::keyword("union");
        lisp += self.name.lispify();
        lisp
    }
}

impl PrettyPrint for UnionFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += self.modifiers.pretty(theme);
        terms += theme.argument(self.field_name.name.to_string(), false);
        terms += theme.keyword(":");
        terms += " ";
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for UnionFieldDeclaration {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
