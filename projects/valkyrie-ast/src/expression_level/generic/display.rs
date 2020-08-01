use super::*;
use crate::PrettyTree;

impl PrettyPrint for GenericCall {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let head = self.base.pretty(allocator);
        let lhs = allocator.text("⦓").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("⦔"));
        let body = self.terms.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        head.append(lhs).append(allocator.concat(body)).append(rhs)
    }
}

impl PrettyPrint for GenericArgumentNode {
    fn pretty<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let lhs = allocator.text("⦓").append(allocator.softline());
        let rhs = allocator.softline().append(allocator.text("⦔"));
        let body = self.terms.iter().map(|x| x.pretty(allocator).append(allocator.softline()));
        lhs.append(allocator.concat(body)).append(rhs)
    }
}
