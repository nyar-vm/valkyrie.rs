use super::*;

impl PrettyPrint for UnionDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("class"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for UnionFieldDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(self.modifiers.build(allocator));
        terms.push(allocator.argument(self.field_name.name.to_string(), false));
        terms.push(allocator.keyword(":"));
        terms.push(allocator.space());
        allocator.concat(terms)
    }
}
