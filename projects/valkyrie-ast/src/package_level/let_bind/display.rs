use super::*;

impl PrettyPrint for LetBindNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(theme.keyword("let"));
        terms.push(theme.space());
        terms.push(self.pattern.build(theme));
        if let Some(type_hint) = &self.type_hint {
            terms.push(theme.text(":"));
            terms.push(theme.space());
            terms.push(type_hint.build(theme));
        }
        if let Some(body) = &self.body {
            terms.push(theme.space());
            terms.push(theme.text("="));
            terms.push(theme.space());
            terms.push(body.build(theme));
        }
        theme.concat(terms)
    }
}
