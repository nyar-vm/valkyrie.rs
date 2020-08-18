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

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ElsePart {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(allocator.hardline());
        terms.push(allocator.keyword("else"));
        terms.push(allocator.space());
        terms.push(allocator.text("{"));
        terms.push(allocator.hardline());
        terms.push(allocator.intersperse(&self.statements, allocator.hardline()).indent(4));
        terms.push(allocator.hardline());
        terms.push(allocator.text("}"));
        allocator.concat(terms)
    }
}
