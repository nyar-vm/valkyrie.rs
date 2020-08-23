use super::*;

impl PrettyPrint for GuardStatement {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(allocator.keyword("guard"));
        terms.push(allocator.space());
        match &self.condition {
            GuardPattern::Inline(e) => {
                terms.push(e.build(allocator));
                terms.push(allocator.space());
            }
            // GuardType::Block(s) => terms.push(s.build(allocator)),
            GuardPattern::Case => {
                todo!()
            }
        }
        terms.push(allocator.keyword("else"));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}
