use super::*;

impl PrettyPrint for TryStatement {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        let mut tree = PrettySequence::new(10);
        tree += theme.keyword("try");
        if let Some(handler) = &self.handler {
            tree += " ";
            tree += handler.pretty(theme);
        }
        tree += " ";
        tree += self.body.pretty(theme);
        tree.into()
    }
}
