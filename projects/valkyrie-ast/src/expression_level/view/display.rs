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
