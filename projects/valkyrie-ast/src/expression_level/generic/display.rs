use super::*;
use alloc::borrow::ToOwned;

// noinspection DuplicatedCode
impl PrettyPrint for GenericCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(allocator.text("⦓"));
        terms.push(allocator.join(&self.terms, ", "));
        terms.push(allocator.text("⦔"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for GenericCallTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        if let Some(k) = &self.term.key {
            terms.push(allocator.generic(k.name.to_owned()));
            terms.push(allocator.text(": "));
        }
        terms.push(self.term.value.build(allocator));
        allocator.concat(terms)
    }
}

// noinspection DuplicatedCode
impl PrettyPrint for GenericArgumentNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(allocator.text("⦓"));
        terms.push(allocator.join(&self.terms, ", "));
        terms.push(allocator.text("⦔"));
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
