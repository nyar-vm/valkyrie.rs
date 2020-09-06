use super::*;
use valkyrie_ast::UnionFieldDeclaration;

impl ThisParser for UnionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("union")(input)?;
        let (state, name) = state.skip(ignore).match_fn(NamePathNode::parse)?;

        state.finish(UnionDeclaration {
            document: Default::default(),
            namepath: name,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            body: Default::default(),
        })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for UnionFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
