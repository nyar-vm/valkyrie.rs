use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for TupleNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            TupleKind::Tuple => {
                let k = KAndRBracket { head_space: false, bracket_l: "(", bracket_r: ")" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            TupleKind::Set => {
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
impl PrettyPrint for TupleTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += match &self.key {
            TupleKeyType::Nothing => return self.value.pretty(theme),
            TupleKeyType::Identifier(node) => node.pretty(theme),
            TupleKeyType::String(node) => theme.number(node.to_string()),
            TupleKeyType::Subscript(node) => node.pretty(theme),
        };
        terms += PrettyTree::text(":");
        terms += self.value.pretty(theme);
        terms.into()
    }
}
