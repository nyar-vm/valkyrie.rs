use super::*;

impl PrettyPrint for RangeNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            RangeKind::Ordinal => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            RangeKind::Offset => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
        }
    }
}
#[cfg(feature = "lispify")]
impl Lispify for RangeNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::unit("ArrayNode ???")
    }
}

impl PrettyPrint for RangeTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            RangeTermNode::Index { index } => index.pretty(theme),
            RangeTermNode::Range { head, tail, step } => {
                let mut terms = PrettySequence::new(5);
                terms += match head {
                    Some(s) => s.pretty(theme),
                    None => PrettyTree::text("1"),
                };
                terms += PrettyTree::text(":");
                terms += match tail {
                    Some(s) => s.pretty(theme),
                    None => PrettyTree::text("-1"),
                };
                terms += PrettyTree::text(":");
                terms += match step {
                    Some(s) => s.pretty(theme),
                    None => PrettyTree::text("1"),
                };
                terms.into()
            }
        }
    }
}
impl PrettyPrint for SubscriptCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(2);
        terms += self.base.pretty(theme);
        match self.kind {
            RangeKind::Ordinal => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                terms += k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            RangeKind::Offset => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                terms += k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
        }
        terms.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for SubscriptCallNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("SubscriptCallNode ???")
    }
}
// impl PrettyPrint for SubscriptTermNode {
//     fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
//         match self {
//             SubscriptTermNode::Index(v) => v.pretty(theme),
//             SubscriptTermNode::Slice(v) => v.pretty(theme),
//         }
//     }
// }
//
// impl PrettyPrint for SubscriptSliceNode {
//     fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
//         let lhs = match &self.start {
//             Some(s) => s.pretty(theme).append(":"),
//             None => ":".into(),
//         };
//         let middle = match &self.end {
//             Some(e) => PrettyTree::text(":").append(e.pretty(theme)),
//             None => " :".into(),
//         };
//         match &self.step {
//             Some(s) => lhs.append(middle).append(s.pretty(theme)),
//             None => lhs.append(middle),
//         }
//     }
// }
