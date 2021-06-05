use super::*;

impl Debug for VariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let w = &mut f.debug_struct("VariableDeclaration");
        w.field("pattern", &self.pattern);
        if let Some(type_hint) = &self.type_hint {
            w.field("type", type_hint);
        }
        if let Some(body) = &self.body {
            w.field("body", body);
        }
        w.field("span", &self.span);
        w.finish()
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for VariableDeclaration {
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
