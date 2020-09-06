use super::*;

impl PrettyPrint for NewConstructNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.keyword("new"));
        for m in &self.modifiers {
            terms.push(allocator.space());
            terms.push(allocator.keyword(m.name.to_string()));
        }
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));

        if !self.generic.terms.is_empty() {
            terms.push(self.generic.build(allocator));
        }
        terms.push(self.arguments.build(allocator));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for CollectsNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut inline = Vec::with_capacity(6);
        inline.push(allocator.space());
        inline.push(allocator.text("{"));
        inline.push(allocator.space());
        inline.push(allocator.join(&self.terms, ", "));
        inline.push(allocator.space());
        inline.push(allocator.text("}"));
        let inline = allocator.concat(inline);
        let mut block = Vec::with_capacity(6);
        block.push(allocator.space());
        block.push(allocator.text("{"));
        block.push(allocator.hardline());
        block.push(allocator.intersperse(&self.terms, allocator.text(",").append(allocator.hardline())).indent(4));
        block.push(allocator.hardline());
        block.push(allocator.text("}"));
        let block = allocator.concat(block);
        inline.flat_alt(block)
    }
}
