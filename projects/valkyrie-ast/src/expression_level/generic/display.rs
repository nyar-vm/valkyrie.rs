use super::*;
use crate::PrettyTree;

impl PrettyPrint for GenericCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = allocator.text("⦓").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("⦔"));
        let body = allocator.intersperse(self.terms.iter().map(|c| c.build(allocator)), ", ");
        lhs.append(body).append(rhs)
    }
}

impl PrettyPrint for GenericCallTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.term.build(allocator)
    }
}

impl PrettyPrint for GenericArgumentNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        if !self.terms.is_empty() {
            terms.push(allocator.text("⦓"));
            terms.push(allocator.intersperse(self.terms.iter().map(|s| s.build(allocator)), allocator.text(", ")));
            terms.push(allocator.text("⦔"));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for GenericArgumentTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.generic(self.term.key.name.to_owned()));
        if let Some(k) = &self.term.value {
            terms.push(allocator.text(": "));
            terms.push(k.build(allocator));
        }
        if let Some(k) = &self.term.default {
            terms.push(allocator.text(" = "));
            terms.push(k.build(allocator));
        }
        allocator.concat(terms)
    }
}
