use super::*;

// noinspection DuplicatedCode
impl PrettyPrint for GenericCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "⦓";
        terms += theme.join(self.terms.clone(), ", ");
        terms += "⦔";
        terms.into()
    }
}

impl PrettyPrint for GenericCallTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(k) = &self.term.key {
            terms += theme.generic(k.name.to_owned());
            terms += ": ";
        }
        terms += self.term.value.pretty(theme);
        terms.into()
    }
}

// noinspection DuplicatedCode
impl PrettyPrint for GenericArgument {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "⦓";
        terms += theme.join(self.terms.clone(), ", ");
        terms += "⦔";
        terms.into()
    }
}

impl Lispify for GenericArgument {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}

impl PrettyPrint for GenericArgumentTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms += theme.generic(self.term.key.name.to_owned());
        if let Some(k) = &self.term.value {
            terms += ": ";
            terms += k.pretty(theme);
        }
        if let Some(k) = &self.term.default {
            terms += " = ";
            terms += k.pretty(theme);
        }
        terms.into()
    }
}
