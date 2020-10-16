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
        terms += self.then_body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for WhileConditionNode {
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
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            WhileConditionNode::Unconditional => theme.keyword("true"),
            WhileConditionNode::Case => theme.keyword("case"),
            WhileConditionNode::Expression(e) => e.pretty(theme),
        }
    }
}

impl PrettyPrint for OtherwiseStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += theme.keyword("otherwise");
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        terms += theme.join(self.terms.clone(), PrettyTree::Hardline).indent(4);
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}
