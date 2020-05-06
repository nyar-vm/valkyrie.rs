use super::*;
use lispify::LispSymbol;

impl Display for ValkyrieSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieSlice {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + 1);
        terms.push(LispSymbol::new("slice").into());
        for term in &self.terms {
            terms.push(term.lispify().into());
        }
        Lisp::Any(terms)
    }
}

impl Lispify for ValkyrieSliceTerm {
    type Output = LispNumber;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
