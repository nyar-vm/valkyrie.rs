use super::*;
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FlagDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms += theme.keyword("enumerate");
        terms += " ";
        terms += self.namepath.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for FlagFieldDeclaration {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.name.pretty(theme);
        if let Some(value) = &self.value {
            terms += " ";
            terms += theme.operator("=");
            terms += " ";
            terms += value.pretty(theme);
        }
        terms += ",";
        terms.into()
    }
}
