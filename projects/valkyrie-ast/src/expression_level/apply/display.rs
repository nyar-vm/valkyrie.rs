use super::*;
use pretty_print::PrettyBuilder;

impl PrettyPrint for ApplyDotNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let newline = PrettyTree::Hardline;
        let mut terms = PrettySequence::new(6);
        terms += ".";
        terms += self.caller.pretty(theme);
        terms += "(";
        terms += theme.join(self.terms.clone(), ", ");
        terms += ")";
        newline.append(terms.indent(4))
    }
}

impl PrettyPrint for ApplyCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms += "(";
        terms += theme.join(self.terms.clone(), ", ");
        terms += ")";
        terms.into()
    }
}

impl PrettyPrint for ApplyCallTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.term.pretty(theme)
    }
}

impl PrettyPrint for ApplyArgument {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        PrettyTree::text("(").append(theme.join(self.terms.clone(), ", ")).append(")")
    }
}
#[cfg(feature = "lispify")]
impl Lispify for ApplyArgument {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        todo!()
    }
}

impl PrettyPrint for ApplyArgumentTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.term.pretty(theme)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mods = self.modifiers.pretty(theme);
        let key = theme.argument(self.key.name.clone(), false);
        mods.append(key)
    }
}
