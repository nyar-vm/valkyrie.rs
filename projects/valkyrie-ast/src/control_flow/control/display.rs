use super::*;

impl Display for ControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for RaiseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms += theme.keyword("raise");
        terms += " ";
        if let Some(s) = &self.expression {
            terms += s.pretty(theme);
        }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ControlNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += self.r#type.pretty(theme);
        if let Some(s) = &self.expression {
            terms += " ";
            terms += s.pretty(theme);
        }
        terms.into()
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for ControlType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(self.as_str())
    }
}
