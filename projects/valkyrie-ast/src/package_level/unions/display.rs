use super::*;

impl PrettyPrint for ModifiersNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(2 * self.terms.len());
        for x in &self.terms {
            items.push(allocator.keyword(x.name.to_string()));
            items.push(allocator.space());
        }
        allocator.concat(items)
    }
}

impl PrettyPrint for UnionDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(8);
        terms.push(self.modifiers.build(allocator));
        terms.push(allocator.keyword("union"));
        terms.push(allocator.space());
        terms.push(self.namepath.build(allocator));
        terms.push(self.statements.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for VariantDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(8);
        terms.push(self.modifiers.build(allocator));
        terms.push(self.namepath.build(allocator));
        terms.push(self.statements.build(allocator));
        allocator.concat(terms)
    }
}
