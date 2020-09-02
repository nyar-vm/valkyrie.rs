
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

impl PrettyPrint for FlagsFieldDeclaration {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(3);
        terms.push(self.name.build(allocator));
        if let Some(value) = &self.value {
            terms.push(allocator.space());
            terms.push(allocator.operator("="));
            terms.push(allocator.space());
            terms.push(value.build(allocator));
            terms.push(allocator.text(","));
        }
        allocator.concat(terms)
    }
}
