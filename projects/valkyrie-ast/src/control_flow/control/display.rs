use super::*;
use pretty_print::helpers::PrettySequence;

impl Display for ControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PrettyPrint for RaiseNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms += theme.keyword("raise");
        terms += " ";
        if let Some(s) = &self.expression {
            terms.push(s.build(theme));
        }
        theme.concat(terms)
    }
}

impl PrettyPrint for ControlNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(self.r#type.build(theme));
        if let Some(s) = &self.expression {
            terms += " ";
            terms += s.pretty(theme);
        }
        terms.into()
    }
}

impl PrettyPrint for ControlType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.keyword(self.as_str())
    }
}
