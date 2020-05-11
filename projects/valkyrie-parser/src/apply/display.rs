use super::*;

impl Display for ValkyrieApply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Lispify for ValkyrieApply {
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

impl Lispify for ValkyrieTableTerm {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            ValkyrieTableTerm::Item(v) => v.lispify(),
            ValkyrieTableTerm::Pair(v) => v.lispify(),
        }
    }
}

impl Lispify for ValkyriePair {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::function("pair"));
        terms.push(self.key.lispify().into());
        terms.push(self.value.lispify().into());
        Lisp::Any(terms)
    }
}
