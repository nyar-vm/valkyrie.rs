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

#[cfg(feature = "lispify")]
impl Lispify for WhileLoop {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(self.then_body.terms.len() + 1);
        match self.kind {
            WhileLoopKind::While => {
                lisp += Lisp::keyword("while");
            }
            WhileLoopKind::Until => {
                lisp += Lisp::keyword("until");
            }
        }
        lisp += self.condition.lispify();
        for term in &self.then_body.terms {
            lisp += term.lispify();
        }
        lisp
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

#[cfg(feature = "lispify")]
impl Lispify for WhileConditionNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            WhileConditionNode::Unconditional => Lisp::keyword("true"),
            WhileConditionNode::Case => Lisp::keyword("case"),
            WhileConditionNode::Expression(v) => v.lispify(),
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
