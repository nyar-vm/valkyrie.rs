use super::*;
use pretty_print::helpers::KAndRBracket;

impl PrettyPrint for TableNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            TableKind::Tuple => {
                let k = KAndRBracket { head_space: false, bracket_l: "(", bracket_r: ")" };
                k.build(&self.terms, theme, theme.text(", "), theme.text(",").append(theme.hardline()))
            }
            TableKind::OffsetTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, theme.text(", "), theme.text(",").append(theme.hardline()))
            }
            TableKind::OrdinalTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "{%", bracket_r: "%}" };
                k.build(&self.terms, theme, theme.text(", "), theme.text(",").append(theme.hardline()))
            }
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.pair.build(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableKeyType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            TableKeyType::Identifier(node) => node.build(theme),
            TableKeyType::Number(node) => node.build(theme),
            TableKeyType::String(node) => node.build(theme),
            TableKeyType::Subscript(node) => node.build(theme),
        }
    }
}
