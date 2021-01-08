use super::*;
use lispify::{Lisp, Lispify};

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

#[cfg(feature = "lispify")]
impl Lispify for IfStatement {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(10);
        lisp += Lisp::keyword("branches");
        for branch in &self.branches {
            lisp += branch.lispify();
        }
        if let Some(else_body) = &self.else_body {
            lisp += else_body.lispify();
        }
        lisp
    }
}

impl PrettyPrint for IfBranchNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += self.condition.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for IfBranchNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}

impl PrettyPrint for ElseStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("else");
        terms += " ";
        terms += self.body.pretty(theme);
        terms.into()
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ElseStatement {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}
