use super::*;
use crate::traits::ThisParser;

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

impl Lispify for TableTermNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            TableTermNode::Item(v) => v.lispify(),
            TableTermNode::Pair(v) => v.lispify(),
        }
    }
}

impl Lispify for PairNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::function("pair"));
        terms.push(self.key.as_lisp().into());
        terms.push(self.value.lispify().into());
        Lisp::Any(terms)
    }
}
