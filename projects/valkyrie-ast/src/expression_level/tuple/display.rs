use super::*;
use pretty_print::helpers::KAndRBracket;

impl PrettyPrint for TupleNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            TupleKind::Tuple => {
                let k = KAndRBracket { head_space: false, bracket_l: "(", bracket_r: ")" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            TupleKind::List => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            TupleKind::Dict => {
                let k = KAndRBracket { head_space: false, bracket_l: "{%", bracket_r: "%}" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
        }
    }
}
#[cfg(feature = "lispify")]
impl Lispify for TupleNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::unit("TupleNode ???")
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TupleKeyType {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            TupleKeyType::Identifier(node) => node.pretty(theme),
            TupleKeyType::Number(node) => node.pretty(theme),
            TupleKeyType::String(node) => node.pretty(theme),
            TupleKeyType::Subscript(node) => node.pretty(theme),
        }
    }
}
#[cfg(feature = "pretty-print")]
impl PrettyPrint for TupleTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.pair.pretty(theme)
    }
}
