use super::*;

impl PrettyPrint for ForLoop {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("for"));
        terms.push(theme.space());
        terms.push(self.pattern.build(theme));
        terms.push(theme.space());
        terms.push(theme.keyword("∈"));
        terms.push(theme.space());
        terms.push(self.iterator.build(theme));
        if let Some(s) = &self.condition {
            terms.push(theme.space());
            terms.push(theme.keyword("if"));
            terms.push(theme.space());
            terms.push(s.build(theme));
        }
        terms.push(self.body.build(theme));
        if let Some(s) = &self.r#else {
            terms.push(s.build(theme));
        }
        theme.concat(terms)
    }
}
