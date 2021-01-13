use super::*;

impl PrettyPrint for GuardStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += theme.keyword("guard");
        terms += " ";
        terms += self.condition.pretty(theme);
        terms += " ";
        terms += self.main_body.pretty(theme);
        terms.into()
    }
}

#[cfg(feature = "lispify")]
impl Lispify for GuardStatement {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        let mut lisp = Lisp::new(4);

        match &self.main_body {
            GuardPattern::Expression(v) => {
                lisp += Lisp::keyword("guard/positive");
                lisp += self.condition.lispify();
                // lisp.extend(v.body.terms.iter().map(|s| s.lispify()));
            }
            GuardPattern::List(v) => {
                lisp += Lisp::keyword("guard/negative");
                lisp += self.condition.lispify();
                lisp.extend(v.body.terms.iter().map(|s| s.lispify()));
            }
            GuardPattern::Dict(v) => {
                lisp += Lisp::keyword("guard/negative");
                lisp += self.condition.lispify();
                lisp.extend(v.body.terms.iter().map(|s| s.lispify()));
            }
        }
        lisp
    }
}

impl PrettyPrint for GuardPattern {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Expression(node) => node.pretty(theme),
            Self::List(node) => node.pretty(theme),
            Self::Dict(node) => node.pretty(theme),
        }
    }
}
