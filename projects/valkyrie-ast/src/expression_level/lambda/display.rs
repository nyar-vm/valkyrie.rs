use super::*;

impl PrettyPrint for LambdaCallNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.build(allocator);
        let lhs = allocator.text("{").append(allocator.hardline());
        let rhs = allocator.hardline().append(allocator.text("}"));
        let body = self.body.iter().map(|x| x.build(allocator).append(allocator.hardline()));
        head.append(allocator.space()).append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl PrettyPrint for LambdaDotNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.build(allocator);
        let lhs = allocator.text(".{").append(allocator.hardline());
        let rhs = allocator.hardline().append(allocator.text("}"));
        let body = self.body.iter().map(|x| x.build(allocator).append(allocator.hardline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}
