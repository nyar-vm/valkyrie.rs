use super::*;
use pretty_print::PrettyBuilder;

impl PrettyPrint for LambdaCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(6);
        terms += " ";
        terms += "{";
        terms += PrettyTree::Hardline;
        terms += theme.join(self.body.clone(), PrettyTree::Hardline.indent(4));
        terms += PrettyTree::Hardline;
        terms += "}";
        terms.into()
    }
}

impl PrettyPrint for LambdaDotNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let newline = PrettyTree::Hardline;
        let mut terms = PrettySequence::new(6);
        terms += ".";
        terms += "{";
        terms += " ";
        terms += theme.join(self.body.clone(), ";");
        terms += " ";
        terms += "}";
        newline.append(terms.indent(4))
    }
}
