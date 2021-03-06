use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ClosureCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(6);
        terms += " ";
        match &self.trailing {
            None => {}
            Some(s) => {
                terms += "{";
                terms += PrettyTree::Hardline;
                // terms += theme.join(self.caller.clone(), PrettyTree::Hardline.indent(4));
                terms += PrettyTree::Hardline;
                terms += "}";
            }
        }
        terms.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for ClosureCallNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        Lisp::keyword("ClosureCallNode ???")
    }
}
