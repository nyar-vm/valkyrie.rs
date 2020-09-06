use super::*;

impl PrettyPrint for FlagsDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("flags"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}
