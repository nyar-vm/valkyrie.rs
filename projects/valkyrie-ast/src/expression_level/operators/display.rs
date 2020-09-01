use super::*;

impl PrettyPrint for OperatorNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.operator(self.kind.as_str())
    }
}

impl PrettyPrint for PrefixNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.operator.build(allocator).append(self.base.build(allocator))
    }
}

impl PrettyPrint for InfixNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut items = Vec::with_capacity(5);
        items.push(self.lhs.build(allocator));
        items.push(allocator.space());
        items.push(self.operator.build(allocator));
        items.push(allocator.space());
        items.push(self.rhs.build(allocator));
        allocator.concat(items)
    }
}

impl PrettyPrint for PostfixNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.base.build(allocator).append(self.operator.build(allocator))
    }
}
