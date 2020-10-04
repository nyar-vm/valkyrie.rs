use super::*;
use pretty_print::helpers::KAndRBracket;

impl PrettyPrint for TableNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            TableKind::Tuple => {
                let k = KAndRBracket { head_space: false, bracket_l: "(", bracket_r: ")" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            TableKind::OffsetTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            TableKind::OrdinalTable => {
                let k = KAndRBracket { head_space: false, bracket_l: "{%", bracket_r: "%}" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.pair.pretty(theme)
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TableKeyType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            TableKeyType::Identifier(node) => node.pretty(theme),
            TableKeyType::Number(node) => node.pretty(theme),
            TableKeyType::String(node) => node.pretty(theme),
            TableKeyType::Subscript(node) => node.pretty(theme),
        }
    }
}
