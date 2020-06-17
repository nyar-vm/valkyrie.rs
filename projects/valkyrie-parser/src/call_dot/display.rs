use super::*;
use crate::traits::ThisParser;

impl Display for ValkyrieDotCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieDotCall {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(self.base.as_lisp().into());
        terms.push(Lisp::Any(vec![Lisp::keyword("."), self.caller.as_lisp().into()]));
        for term in &self.terms {
            terms.push(term.as_lisp().into());
        }
        Lisp::Any(terms)
    }
}
