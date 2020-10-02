use super::*;

impl PrettyPrint for ModifiersNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut items = PrettySequence::new(2 * self.terms.len());
        for x in &self.terms {
            items.push(theme.keyword(x.name.to_string()));
            items.push(" ");
        }
        items.into()
    }
}

impl PrettyPrint for TaggedDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(8);
        terms += self.modifiers.pretty(theme);
        terms += theme.keyword("enumerate");
        terms += " ";
        terms += self.namepath.pretty(theme);
        terms += self.statements.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for VariantDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(8);
        // terms += self.modifiers.pretty(theme);
        terms += theme.argument(self.variant.name.to_string(), false);
        terms += self.statements.pretty(theme);
        terms.into()
    }
}
