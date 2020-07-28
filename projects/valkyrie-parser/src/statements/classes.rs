use super::*;

impl ThisParser for ClassDeclarationNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("class")?;
        let (state, namepath) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, _) = state.skip(ignore).match_char('{')?;
        let (state, _) = state.skip(ignore).match_char('}')?;
        state.finish(ClassDeclarationNode {
            namepath,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            statements: vec![],
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(4);
        items.push(Lisp::keyword("class"));
        items.push(self.namepath.as_lisp());
        Lisp::Any(items)
    }
}
