use super::*;

impl PrettyPrint for EnumerateDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("enumerate"));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for EnumerateFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(self.name.build(theme));
        if let Some(value) = &self.value {
            terms.push(theme.space());
            terms.push(theme.operator("="));
            terms.push(theme.space());
            terms.push(value.build(theme));
        }
        terms.push(theme.text(","));
        theme.concat(terms)
    }
}
