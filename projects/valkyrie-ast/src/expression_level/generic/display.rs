use super::*;

// noinspection DuplicatedCode
impl PrettyPrint for GenericCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(theme.text("⦓"));
        terms.push(theme.join(&self.terms, ", "));
        terms.push(theme.text("⦔"));
        theme.concat(terms)
    }
}

impl PrettyPrint for GenericCallTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        if let Some(k) = &self.term.key {
            terms.push(theme.generic(k.name.to_owned()));
            terms.push(theme.text(": "));
        }
        terms.push(self.term.value.build(theme));
        theme.concat(terms)
    }
}

// noinspection DuplicatedCode
impl PrettyPrint for GenericArgumentNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(theme.text("⦓"));
        terms.push(theme.join(&self.terms, ", "));
        terms.push(theme.text("⦔"));
        theme.concat(terms)
    }
}

impl PrettyPrint for GenericArgumentTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms.push(theme.generic(self.term.key.name.to_owned()));
        if let Some(k) = &self.term.value {
            terms.push(theme.text(": "));
            terms.push(k.build(theme));
        }
        if let Some(k) = &self.term.default {
            terms.push(theme.text(" = "));
            terms.push(k.build(theme));
        }
        theme.concat(terms)
    }
}
