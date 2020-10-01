use super::*;

impl PrettyPrint for ModifiersNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = Vec::with_capacity(2 * self.terms.len());
        for x in &self.terms {
            items.push(theme.keyword(x.name.to_string()));
            items.push(theme.space());
        }
        theme.concat(items)
    }
}

impl PrettyPrint for TaggedDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(8);
        terms.push(self.modifiers.build(theme));
        terms.push(theme.keyword("enumerate"));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        terms.push(self.statements.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for VariantDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(8);
        // terms.push(self.modifiers.build(theme));
        terms.push(theme.argument(self.variant.name.to_string(), false));
        terms.push(self.statements.build(theme));
        theme.concat(terms)
    }
}
