use super::*;

impl PrettyPrint for LambdaCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(6);
        terms.push(allocator.space());
        terms.push(allocator.text("{"));
        terms.push(allocator.hardline());
        terms.push(allocator.intersperse(&self.body, allocator.hardline()).indent(4));
        terms.push(allocator.hardline());
        terms.push(allocator.text("}"));
        allocator.concat(terms)
    }
}

impl PrettyPrint for LambdaDotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let newline = allocator.hardline();
        let mut terms = Vec::with_capacity(6);
        terms.push(allocator.text("."));
        terms.push(allocator.text("{"));
        terms.push(allocator.space());
        terms.push(allocator.join(&self.body, ";"));
        terms.push(allocator.space());
        terms.push(allocator.text("}"));
        newline.append(allocator.concat(terms).indent(4))
    }
}
