use super::*;

impl ValkyrieNode for MatchStatement {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

impl ValkyrieNode for MatchCallNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

#[cfg(feature = "pretty-print")]
impl PrettyPrint for MatchCallNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut terms = PrettySequence::new(10);
        terms += PrettyTree::Hardline;
        terms += self.monadic.pretty(theme);
        terms += theme.keyword(self.kind.as_str());
        terms += " ";
        terms += self.patterns.pretty(theme);
        terms.into()
    }
}
