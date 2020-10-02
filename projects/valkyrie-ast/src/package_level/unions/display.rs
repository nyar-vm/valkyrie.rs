use super::*;

impl PrettyPrint for UnionDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("union");
        terms += " ";
        terms += self.namepath.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
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
