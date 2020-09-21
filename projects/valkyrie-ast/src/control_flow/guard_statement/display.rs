use super::*;

impl PrettyPrint for GuardStatement {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(allocator.keyword("guard"));
        terms.push(allocator.space());
        terms.push(self.condition.build(allocator));
        terms.push(allocator.space());
        terms.push(allocator.keyword("else"));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for GuardPattern {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            GuardPattern::Case(e) => e.build(allocator),
            GuardPattern::Inline(e) => e.build(allocator),
        }
    }
}
