use super::*;

impl PrettyPrint for IfStatement {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(self.branches.len() * 4 + 3);
        for (idx, term) in self.branches.iter().enumerate() {
            if idx == 0 {
                terms.push(allocator.keyword("when "));
                terms.push(term.condition.build(allocator));
            }
            else {
                terms.push(allocator.hardline());
                terms.push(allocator.keyword("else if "));
                terms.push(term.condition.build(allocator));
            }
            terms.push(term.body.build(allocator));
        }
        terms.push(self.else_branch.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for IfConditionNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(self.condition.build(allocator));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
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
impl PrettyPrint for ElseStatement {
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
