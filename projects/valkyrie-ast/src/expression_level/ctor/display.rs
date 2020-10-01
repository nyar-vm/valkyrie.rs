use super::*;

impl PrettyPrint for NewConstructNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms.push(theme.keyword("new"));
        for m in &self.modifiers {
            terms.push(theme.space());
            terms.push(theme.keyword(m.name.to_string()));
        }
        terms.push(theme.space());
        terms.push(self.namepath.build(theme));

        if !self.generic.terms.is_empty() {
            terms.push(self.generic.build(theme));
        }
        terms.push(self.arguments.build(theme));
        terms.push(self.body.build(theme));
        theme.concat(terms)
    }
}

impl PrettyPrint for CollectsNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut inline = Vec::with_capacity(6);
        inline.push(theme.space());
        inline.push(theme.text("{"));
        inline.push(theme.space());
        inline.push(theme.join(&self.terms, ", "));
        inline.push(theme.space());
        inline.push(theme.text("}"));
        let inline = theme.concat(inline);
        let mut block = Vec::with_capacity(6);
        block.push(theme.space());
        block.push(theme.text("{"));
        block.push(theme.hardline());
        block.push(theme.intersperse(&self.terms, theme.text(",").append(theme.hardline())).indent(4));
        block.push(theme.hardline());
        block.push(theme.text("}"));
        let block = theme.concat(block);
        inline.flat_alt(block)
    }
}
