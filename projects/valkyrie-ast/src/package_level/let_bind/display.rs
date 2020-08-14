use super::*;

impl PrettyPrint for LetBindNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(allocator.keyword("let"));
        terms.push(allocator.space());
        terms.push(self.pattern.build(allocator));
        if let Some(type_hint) = &self.type_hint {
            terms.push(allocator.text(":"));
            terms.push(allocator.space());
            terms.push(type_hint.build(allocator));
        }
        if let Some(body) = &self.body {
            terms.push(allocator.space());
            terms.push(allocator.text("="));
            terms.push(allocator.space());
            terms.push(body.build(allocator));
        }
        allocator.concat(terms)
    }
}
