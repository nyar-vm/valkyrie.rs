use super::*;

impl ControlKind {
    /// Convert to keywords
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Goto => "goto",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Fallthrough => "fallthrough",
            Self::Return => "return",
            Self::Resume => "resume",
            Self::Yield => "yield",
            Self::YieldReturn => "yield return",
            Self::YieldBreak => "yield break",
            Self::YieldFrom => "yield from",
        }
    }
}

impl Display for ControlKind {
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
impl PrettyPrint for ControlKind {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(self.as_str())
    }
}
