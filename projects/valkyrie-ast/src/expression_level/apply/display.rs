use super::*;

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

impl PrettyPrint for ApplyCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(allocator.text("("));
        terms.push(allocator.intersperse(self.terms.iter().map(|x| x.build(allocator)), allocator.text(", ")));
        terms.push(allocator.text(")"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ApplyArgumentNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator
            .text("(")
            .append(allocator.intersperse(self.terms.iter().map(|x| x.build(allocator)), allocator.text(", ")))
            .append(allocator.text(")"))
    }
}

impl PrettyPrint for ApplyArgumentTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.term.build(allocator)
    }
}

impl PrettyPrint for ApplyCallTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.term.build(allocator)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mods = allocator.concat(self.modifiers.iter().map(|s| allocator.keyword(s.name.clone()).append(allocator.space())));
        let key = allocator.argument(self.key.name.clone());
        mods.append(key)
    }
}
