use super::*;
use crate::PrettyTree;
use pretty::Pretty;

impl PrettyPrint for ApplyDotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.build(allocator);
        let lhs = allocator
            .text(".")
            .append(allocator.text(self.caller.name.clone()))
            .append(allocator.text("("))
            .append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text(")"));
        let body = self.terms.iter().map(|x| x.build(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl<K, V> PrettyPrint for ApplyTermNode<K, V>
where
    K: PrettyPrint,
    V: PrettyPrint,
{
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        if let Some(k) = &self.key {
            terms.push(k.build(allocator));
            terms.push(allocator.text(": "));
        }
        terms.push(self.value.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ApplyCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.base.build(allocator));
        terms.push(allocator.text("("));
        terms.push(allocator.intersperse(self.terms.iter().map(|x| x.build(allocator)), allocator.text(", ")));
        terms.push(allocator.text(")"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mods = allocator.concat(self.modifiers.iter().map(|s| allocator.keyword(s.name.clone()).append(allocator.space())));
        let key = allocator.argument(self.key.name.clone());
        mods.append(key)
    }
}
