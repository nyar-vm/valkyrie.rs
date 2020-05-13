use super::*;

impl Display for ValkyrieDotCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieDotCall {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
        // let mut terms = Vec::with_capacity(self.caller.len() + 2);
        // terms.push(Lisp::function("index"));
        // terms.push(self.base.lispify().into());
        // for term in &self.caller {
        //     terms.push(term.lispify().into());
        // }
        // Lisp::Any(terms)
    }
}
