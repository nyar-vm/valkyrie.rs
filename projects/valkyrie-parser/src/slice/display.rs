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
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieSliceTerm::Single { element, range } => return element.lispify().into(),
            ValkyrieSliceTerm::Ranged { start, end, step, range } => {
                let mut terms = Vec::with_capacity(4);
                terms.push(LispSymbol::new("range").into());
                match start {
                    None => terms.push(Lisp::keyword("nil")),
                    Some(s) => terms.push(s.lispify().into()),
                }
                match end {
                    None => terms.push(Lisp::keyword("nil")),
                    Some(s) => terms.push(s.lispify().into()),
                }
                match step {
                    None => terms.push(Lisp::keyword("nil")),
                    Some(s) => terms.push(s.lispify().into()),
                }
                Lisp::Any(terms)
            }
        }
    }
}
