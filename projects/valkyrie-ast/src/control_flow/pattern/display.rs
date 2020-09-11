use crate::{PatternCaseNode, PatternCondition, PatternElseNode, PatternTypeNode, PatternWhenNode};
use alloc::{vec, vec::Vec};
use pretty_print::{PrettyPrint, PrettyProvider, PrettyTree};

impl PrettyPrint for PatternCondition {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let item = match self {
            Self::Case(v) => v.build(allocator),
            Self::When(v) => v.build(allocator),
            Self::Type(v) => v.build(allocator),
            Self::Else(v) => v.build(allocator),
        };
        item.append(allocator.text(":")).append(allocator.hardline())
    }
}

impl PrettyPrint for PatternCaseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.keyword("case"));
        terms.push(allocator.space());
        terms.push(self.pattern.build(allocator));
        if let Some(guard) = &self.guard {
            terms.push(allocator.keyword("when"));
            terms.push(guard.build(allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for PatternWhenNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.concat(vec![allocator.keyword("when"), allocator.space(), self.guard.build(allocator)])
    }
}

impl PrettyPrint for PatternTypeNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.concat(vec![allocator.keyword("type"), allocator.space(), self.pattern.build(allocator)])
    }
}

impl PrettyPrint for PatternElseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword("else")
    }
}
