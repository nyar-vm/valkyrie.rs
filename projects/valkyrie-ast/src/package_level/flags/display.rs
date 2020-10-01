use super::*;

impl PrettyPrint for FlagsDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("flags"));
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}
