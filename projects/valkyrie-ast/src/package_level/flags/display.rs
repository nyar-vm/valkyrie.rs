use super::*;

impl PrettyPrint for FlagsDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("flags");
        terms += " ";
        terms += self.namepath.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}
