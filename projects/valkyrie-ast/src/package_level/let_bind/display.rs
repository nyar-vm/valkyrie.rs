use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for LetBindNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += theme.keyword("let");
        terms += " ";
        terms += self.pattern.pretty(theme);
        if let Some(type_hint) = &self.type_hint {
            terms += ":";
            terms += " ";
            terms += type_hint.pretty(theme);
        }
        if let Some(body) = &self.body {
            terms += " ";
            terms += "=";
            terms += " ";
            terms += body.pretty(theme);
        }
        terms.into()
    }
}
