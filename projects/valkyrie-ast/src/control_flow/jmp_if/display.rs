use super::*;
use crate::SoftBlock;

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
impl PrettyPrint for ElseStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += theme.keyword("else");
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        terms += theme.join(self.statements.clone(), PrettyTree::Hardline).indent(4);
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}

impl PrettyPrint for ThenStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += theme.keyword("then");
        terms += " ";
        terms += SoftBlock::curly_braces().join_slice(&self.statements, theme);
        terms.into()
    }
}

impl PrettyPrint for IfLetStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("if let ");
        terms += self.pattern.pretty(theme);
        terms += self.then_body.pretty(theme);
        if let Some(else_body) = &self.else_body {
            terms += else_body.pretty(theme);
        }
        terms.into()
    }
}
