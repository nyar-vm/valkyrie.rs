use super::*;

impl PrettyPrint for PatternBranch {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.concat(vec![self.condition.build(allocator), self.statements.build(allocator)])
    }
}

impl PrettyPrint for PatternCondition {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let item = match self {
            Self::Case(v) => v.build(allocator),
            Self::When(v) => v.build(allocator),
            Self::Type(v) => v.build(allocator),
            Self::Else(v) => v.build(allocator),
        };
        item.append(allocator.text(":")).append(allocator.hardline())
    }
}

impl PrettyPrint for PatternStatements {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(self.terms.len() * 2);
        let len = self.terms.len();
        for (idx, term) in self.terms.iter().enumerate() {
            terms.push(term.build(allocator));
            if idx == len.saturating_sub(1) {
                terms.push(allocator.text(","));
            }
        }
        allocator.concat(terms).indent(4)
    }
}

impl PrettyPrint for ImplicitCaseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(10);
        terms.push(self.pattern.build(allocator));
        terms.push(allocator.keyword("≔"));
        terms.push(self.body.build(allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for PatternCaseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(5);
        terms.push(allocator.keyword("case"));
        terms.push(allocator.space());
        terms.push(self.pattern.build(allocator));
        if let Some(guard) = &self.guard {
            terms.push(allocator.keyword("when"));
            terms.push(guard.build(allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for PatternWhenNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.concat(vec![allocator.keyword("when"), allocator.space(), self.guard.build(allocator)])
    }
}

impl PrettyPrint for PatternTypeNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.concat(vec![allocator.keyword("type"), allocator.space(), self.pattern.build(allocator)])
    }
}

impl PrettyPrint for PatternElseNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        allocator.keyword("else")
    }
}
