use super::*;
use crate::ElsePart;

impl PrettyPrint for WhileLoopNode {
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
        terms.push(FunctionBodyPart::build_borrowed(&self.body, allocator));
        allocator.concat(terms)
    }
}

impl PrettyPrint for ForLoopNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        let mut terms = Vec::with_capacity(4);
        terms.push(allocator.keyword("for"));
        terms.push(allocator.space());
        terms.push(self.pattern.build(allocator));
        terms.push(allocator.space());
        terms.push(allocator.keyword("∈"));
        terms.push(allocator.space());
        terms.push(self.iterator.build(allocator));
        if !self.condition.is_empty() {
            terms.push(allocator.space());
            terms.push(allocator.keyword("if"));
            terms.push(allocator.space());
            terms.push(self.condition.build(allocator));
        }
        terms.push(FunctionBodyPart::build_borrowed(&self.body, allocator));
        if !self.r#else.is_empty() {
            terms.push(ElsePart::build_borrowed(&self.body, allocator));
        }
        allocator.concat(terms)
    }
}

impl PrettyPrint for PatternType {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            PatternType::Tuple(v) => {
                let mut terms = Vec::with_capacity(4);
                terms.push(allocator.text("("));
                terms.push(allocator.join(v, ", "));
                terms.push(allocator.text(")"));
                allocator.concat(terms)
            }
            PatternType::Case => allocator.keyword("case"),
        }
    }
}
