use super::*;

impl PrettyPrint for ForLoop {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("for");
        terms += " ";
        terms += self.pattern.pretty(theme);
        terms += " ";
        terms += theme.keyword("∈");
        terms += " ";
        terms += self.iterator.pretty(theme);
        if let Some(s) = &self.condition {
            terms += " ";
            terms += theme.keyword("if");
            terms += " ";
            terms += s.pretty(theme);
        }
        terms += self.then_body.pretty(theme);
        if let Some(s) = &self.else_body {
            terms += s.pretty(theme);
        }
        terms.into()
    }
}
