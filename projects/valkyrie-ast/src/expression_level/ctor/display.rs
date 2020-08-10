use super::*;

impl PrettyPrint for NewConstructNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.keyword("new"));
        for m in &self.modifiers {
            terms.push(allocator.space());
            terms.push(m.build(allocator));
        }
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));

        if !self.generic.terms.is_empty() {
            terms.push(self.generic.build(allocator));
        }
        terms.push(self.arguments.build(allocator));
        if !self.collectors.is_empty() {
            let mut inline = Vec::with_capacity(6);
            inline.push(allocator.space());
            inline.push(allocator.text("{"));
            inline.push(allocator.space());
            inline.push(allocator.join(&self.collectors, ", "));
            inline.push(allocator.space());
            inline.push(allocator.text("}"));
            let mut block = Vec::with_capacity(6);
            block.push(allocator.space());
            block.push(allocator.text("{"));
            block.push(allocator.hardline());
            block.push(allocator.intersperse(&self.collectors, allocator.text(",").append(allocator.hardline())).indent(4));
            block.push(allocator.hardline());
            block.push(allocator.text("}"));
            terms.push(allocator.concat(block).flat_alt(allocator.concat(inline)))
        }
        allocator.concat(terms)
    }
}
