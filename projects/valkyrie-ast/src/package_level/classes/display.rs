use super::*;

impl PrettyPrint for ClassDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("class"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        allocator.concat(terms)
    }
}
