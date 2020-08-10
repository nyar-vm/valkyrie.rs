use super::*;

impl PrettyPrint for ApplyDotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let newline = allocator.hardline();
        let mut terms = Vec::with_capacity(6);
        terms.push(allocator.text("."));
        terms.push(allocator.identifier(self.caller.name.clone()));
        terms.push(allocator.text("("));
        terms.push(allocator.join(&self.terms, ", "));
        terms.push(allocator.text(")"));
        newline.append(allocator.concat(terms).indent(4))
    }
}

impl PrettyPrint for ApplyCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(allocator.text("("));
        terms.push(allocator.join(&self.terms, ", "));
        terms.push(allocator.text(")"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ApplyCallTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.term.build(allocator)
    }
}

impl PrettyPrint for ApplyArgumentNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.text("(").append(allocator.join(&self.terms, ", ")).append(allocator.text(")"))
    }
}

impl PrettyPrint for ApplyArgumentTerm {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.term.build(allocator)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mods = allocator.concat(self.modifiers.iter().map(|s| allocator.keyword(s.name.clone()).append(allocator.space())));
        let key = allocator.argument(self.key.name.clone(), false);
        mods.append(key)
    }
}
