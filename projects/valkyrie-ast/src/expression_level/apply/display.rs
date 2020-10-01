use super::*;

impl PrettyPrint for ApplyDotNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let newline = theme.hardline();
        let mut terms = PrettySequence::new(6);
        terms.push(theme.text("."));
        terms.push(self.caller.build(theme));
        terms.push(theme.text("("));
        terms.push(theme.join(&self.terms, ", "));
        terms.push(theme.text(")"));
        newline.append(theme.concat(terms).indent(4))
    }
}

impl PrettyPrint for ApplyCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(3);
        terms.push(theme.text("("));
        terms.push(theme.join(&self.terms, ", "));
        terms.push(theme.text(")"));
        theme.concat(terms)
    }
}

impl PrettyPrint for ApplyCallTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.term.build(theme)
    }
}

impl PrettyPrint for ApplyArgumentNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        theme.text("(").append(theme.join(&self.terms, ", ")).append(theme.text(")"))
    }
}

impl PrettyPrint for ApplyArgumentTerm {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        self.term.build(theme)
    }
}

impl PrettyPrint for ArgumentKeyNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mods = self.modifiers.build(theme);
        let key = theme.argument(self.key.name.clone(), false);
        mods.append(key)
    }
}
