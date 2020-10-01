use super::*;

impl PrettyPrint for UnionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("union"));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for UnionFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(self.modifiers.build(theme));
        terms.push(theme.argument(self.field_name.name.to_string(), false));
        terms.push(theme.keyword(":"));
        terms.push(theme.space());
        theme.concat(terms)
    }
}
