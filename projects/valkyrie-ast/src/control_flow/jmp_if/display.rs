use super::*;

impl PrettyPrint for IfStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(self.branches.len() * 4 + 3);
        for (idx, term) in self.branches.iter().enumerate() {
            if idx == 0 {
                terms.push(theme.keyword("if "));
                terms.push(term.condition.build(theme));
            }
            else {
                terms.push(theme.hardline());
                terms.push(theme.keyword("else if "));
                terms.push(term.condition.build(theme));
            }
            terms.push(term.body.build(theme));
        }
        terms.push(self.else_branch.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for IfConditionNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms.push(self.condition.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
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
            WhileConditionNode::AlwaysTrue => theme.keyword("true"),
            WhileConditionNode::Case => theme.keyword("case"),
            WhileConditionNode::Expression(e) => e.build(theme),
        }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for ElseStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms.push(theme.hardline());
        terms.push(theme.keyword("else"));
        terms.push(theme.space());
        terms.push(theme.text("{"));
        terms.push(theme.hardline());
        terms.push(theme.intersperse(&self.statements, theme.hardline()).indent(4));
        terms.push(theme.hardline());
        terms.push(theme.text("}"));
        theme.concat(terms)
    }
}
