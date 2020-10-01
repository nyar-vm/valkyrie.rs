use super::*;

impl PrettyPrint for LambdaCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(6);
        terms.push(theme.space());
        terms.push(theme.text("{"));
        terms.push(theme.hardline());
        terms.push(theme.intersperse(&self.body, theme.hardline()).indent(4));
        terms.push(theme.hardline());
        terms.push(theme.text("}"));
        theme.concat(terms)
    }
}

impl PrettyPrint for LambdaDotNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let newline = theme.hardline();
        let mut terms = PrettySequence::new(6);
        terms.push(theme.text("."));
        terms.push(theme.text("{"));
        terms.push(theme.space());
        terms.push(theme.join(&self.body, ";"));
        terms.push(theme.space());
        terms.push(theme.text("}"));
        newline.append(theme.concat(terms).indent(4))
    }
}
