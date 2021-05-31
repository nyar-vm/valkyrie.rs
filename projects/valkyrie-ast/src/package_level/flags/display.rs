use super::*;

impl Debug for FlagTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Macro(v) => Debug::fmt(v, f),
            Self::Encode(v) => Debug::fmt(v, f),
            Self::Method(v) => Debug::fmt(v, f),
        }
    }
}

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
impl PrettyPrint for EncodeDeclaration {
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
