use super::*;

impl PrettyPrint for WhileLoop {
    /// ```vk
    /// # inline style
    /// while a || b || c { ... }
    ///
    /// # block style
    /// while a
    ///     || b && c
    ///     && d || e
    /// {
    ///    ...
    /// }
    /// ```
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("while"));
        terms.push(allocator.space());
        terms.push(self.condition.build(allocator));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for PatternExpressionNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            Self::Tuple(v) => {
                let mut terms = Vec::with_capacity(4);
                terms.push(allocator.text("("));
                terms.push(allocator.join(v, ", "));
                terms.push(allocator.text(")"));
                allocator.concat(terms)
            }
            Self::Case => allocator.keyword("case"),
        }
    }
}
