use super::*;
use crate::PrettyTree;

impl PrettyPrint for LambdaCallNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.pretty(allocator);
        let lhs = allocator.text("{").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("}"));
        let body = self.body.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        head.append(allocator.space()).append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl PrettyPrint for LambdaDotNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.pretty(allocator);
        let lhs = allocator.text(".{").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("}"));
        let body = self.body.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}
