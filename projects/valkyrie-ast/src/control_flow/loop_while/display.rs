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
        terms += theme.keyword("while");
        terms += " ";
        terms += self.condition.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for PatternExpressionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Tuple(v) => {
                let mut terms = PrettySequence::new(4);
                terms += "(";
                terms += theme.join(v, ", ");
                terms += ")";
                terms.into()
            }
            Self::Case => theme.keyword("case"),
        }
    }
}
