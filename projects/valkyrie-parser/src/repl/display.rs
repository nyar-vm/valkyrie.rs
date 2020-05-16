use super::*;

impl Display for ValkyrieREPL {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieREPL {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("index"));
        terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.lispify().into());
        }
        Lisp::Any(terms)
    }
}

impl Lispify for ValkyrieViewTerm {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieViewTerm::Index { element, .. } => return element.lispify().into(),
            ValkyrieViewTerm::Range { start, end, step, .. } => {
                let mut terms = Vec::with_capacity(4);
                terms.push(Lisp::function("range").into());
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
