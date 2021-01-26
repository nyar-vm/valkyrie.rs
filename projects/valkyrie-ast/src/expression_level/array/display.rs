use super::*;

impl PrettyPrint for SubscriptCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        // let lhs = if self.kind { "⁅" } else { "[" };
        // let terms = theme.join(self.terms.clone(), ", ");
        // let rhs = if self.kind { "⁆" } else { "]" };
        // PrettyTree::StaticText(lhs).append(terms).append(rhs)
        theme.text("SubscriptCallNode ???")
    }
}

#[cfg(feature = "lispify")]
impl Lispify for SubscriptCallNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("SubscriptCallNode ???")
    }
}
impl PrettyPrint for ArrayNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self.kind {
            ArrayKind::Ordinal => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
            ArrayKind::Offset => {
                let k = KAndRBracket { head_space: false, bracket_l: "[", bracket_r: "]" };
                k.build(&self.terms, theme, ", ".into(), PrettyTree::text(",").append(PrettyTree::Hardline))
            }
        }
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ArrayNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::unit("ArrayNode ???")
    }
}

impl PrettyPrint for ArrayTermNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            ArrayTermNode::Index { index } => index.pretty(theme),
            ArrayTermNode::Range { head, tail, step } => theme.text("ArrayTermNode::Range ???"),
        }
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
