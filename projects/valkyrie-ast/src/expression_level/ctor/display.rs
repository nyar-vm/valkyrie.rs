use super::*;
use pretty_print::PrettyBuilder;

impl PrettyPrint for NewConstructNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms += theme.keyword("new");
        for m in &self.modifiers {
            terms += " ";
            terms += theme.keyword(m.name.to_string());
        }
        terms += " ";
        terms += self.namepath.pretty(theme);

        if !self.generic.terms.is_empty() {
            terms += self.generic.pretty(theme);
        }
        terms += self.arguments.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for CollectsNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut inline = PrettySequence::new(6);
        inline += " ";
        inline += "{";
        inline += " ";
        inline += theme.join(&self.terms, ", ");
        inline += " ";
        inline += "}";
        let mut block = PrettySequence::new(6);
        block += " ";
        block += "{";
        block += PrettyTree::Hardline;
        block += theme.join(&self.terms, ",").append(PrettyTree::Hardline).indent(4);
        block += PrettyTree::Hardline;
        block += "}";
        inline.flat_alt(block)
    }
}
