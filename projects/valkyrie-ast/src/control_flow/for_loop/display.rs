use super::*;

impl PrettyPrint for ForLoop {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("for"));
        terms.push(allocator.space());
        terms.push(self.pattern.build(allocator));
        terms.push(allocator.space());
        terms.push(allocator.keyword("∈"));
        terms.push(allocator.space());
        terms.push(self.iterator.build(allocator));
        if let Some(s) = &self.condition {
            terms.push(allocator.space());
            terms.push(allocator.keyword("if"));
            terms.push(allocator.space());
            terms.push(s.build(allocator));
        }
        terms.push(self.body.build(allocator));
        if let Some(s) = &self.r#else {
            terms.push(s.build(allocator));
        }
        allocator.concat(terms)
    }
}
