use super::*;
use pretty_print::PrettyBuilder;

impl PrettyPrint for ConstructNewNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(5);
        terms += theme.keyword("new");
        for m in &self.annotations {
            terms += " ";
            terms += theme.keyword(m.name.to_string());
        }
        terms += " ";
        terms += self.namepath.pretty(theme);

        if !self.generics.terms.is_empty() {
            terms += self.generics.pretty(theme);
        }
        terms += self.arguments.pretty(theme);
        terms += self.body.pretty(theme);
        terms.into()
    }
}

impl PrettyPrint for CollectorNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut inline = PrettySequence::new(6);
        inline += " ";
        inline += "{";
        inline += " ";
        inline += theme.join(self.terms.clone(), ", ");
        inline += " ";
        inline += "}";
        let mut block = PrettySequence::new(6);
        block += " ";
        block += "{";
        block += PrettyTree::Hardline;
        block += theme.join(self.terms.clone(), ",").append(PrettyTree::Hardline).indent(4);
        block += PrettyTree::Hardline;
        block += "}";
        inline.flat_alt(block)
    }
}
