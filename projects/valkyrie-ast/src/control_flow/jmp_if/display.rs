use super::*;

impl PrettyPrint for IfStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.branches.len() * 4 + 3);
        for (idx, term) in self.branches.iter().enumerate() {
            if idx == 0 {
                terms += theme.keyword("if ");
                terms += term.condition.pretty(theme);
            }
            else {
                terms += PrettyTree::Hardline;
                terms += theme.keyword("else if ");
                terms += term.condition.pretty(theme);
            }
            terms += term.body.pretty(theme);
        }
        terms += match &self.else_body {
            Some(s) => s.pretty(theme),
            None => ElseStatement::default().pretty(theme),
        };
        terms.into()
    }
}

impl PrettyPrint for IfConditionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += self.condition.pretty(theme);
        terms += self.body.pretty(theme);
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

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ElseStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += theme.keyword("else");
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        terms += theme.join(&self.statements, PrettyTree::Hardline).indent(4);
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}
