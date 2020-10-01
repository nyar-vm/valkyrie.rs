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
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(4);
        terms.push(theme.keyword("while"));
        terms.push(theme.space());
        terms.push(self.condition.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for PatternExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Tuple(v) => {
                let mut terms = PrettySequence::new(4);
                terms.push(theme.text("("));
                terms.push(theme.join(v, ", "));
                terms.push(theme.text(")"));
                theme.concat(terms)
            }
            Self::Case => theme.keyword("case"),
        }
    }
}
