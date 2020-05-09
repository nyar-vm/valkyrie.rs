use super::*;

impl Display for ValkyrieTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieTable {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("table"));
        for term in &self.terms {
            terms.push(term.lispify().into());
        }
        Lisp::Any(terms)
    }
}
