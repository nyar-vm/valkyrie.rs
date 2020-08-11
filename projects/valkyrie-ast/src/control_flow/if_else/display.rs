use super::*;

impl PrettyPrint for IfStatementNode {
    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for ConditionNode {
    fn build<'a>(&self, _allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        todo!()
    }
}

impl PrettyPrint for ConditionType {
    /// ```vk
    /// # inline style
    /// a || b || c
    ///
    /// # block style
    ///
    /// a
    ///   || b && c
    ///   && d || e
    /// ```
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            ConditionType::AlwaysTrue => allocator.keyword("true"),
            ConditionType::Case => allocator.keyword("case"),
            ConditionType::Expression(e) => e.build(allocator),
        }
    }
}

impl<'i> PrettyPrint for ElsePart<'i> {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(2);
        terms.push(allocator.keyword("else"));
        let head = allocator.text("{");
        let body = self.body.iter().map(|x| x.build(allocator).append(allocator.text(",")));
        let tail = allocator.text("}");
        let table = head.append(allocator.concat(body)).append(tail);
        terms.push(table);
        allocator.concat(terms)
    }
}
