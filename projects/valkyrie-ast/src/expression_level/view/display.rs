use super::*;
use crate::PrettyTree;

impl PrettyPrint for SubscriptNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = allocator.text(if self.index0 { "[" } else { "⁅" });
        let terms = allocator.intersperse(self.terms.iter().map(|item| item.build(allocator)), allocator.text(", "));
        let rhs = allocator.text(if self.index0 { "]" } else { "⁆" });
        lhs.append(terms).append(rhs)
    }
}

impl PrettyPrint for SubscriptTermNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            SubscriptTermNode::Index(v) => v.build(allocator),
            SubscriptTermNode::Slice(v) => v.build(allocator),
        }
    }
}

impl PrettyPrint for SubscriptSliceNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = match &self.start {
            Some(s) => s.build(allocator).append(allocator.text(":")),
            None => allocator.text(":"),
        };
        let middle = match &self.end {
            Some(e) => allocator.text(":").append(e.build(allocator)),
            None => allocator.text(" :"),
        };
        match &self.step {
            Some(s) => lhs.append(middle).append(s.build(allocator)),
            None => lhs.append(middle),
        }
    }
}
