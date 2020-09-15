use super::*;

impl ThisParser for UnionDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("union")(input)?;
        let (state, name) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), union_statements)?;

        state.finish(UnionDeclaration {
            document: Default::default(),
            namepath: name,
            modifiers: vec![],
            extends: None,
            implements: vec![],
            body: stmt,
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut items = Vec::with_capacity(4);
        items.push(Lisp::keyword("union"));
        items.push(self.namepath.as_lisp());
        Lisp::Any(items)
    }
}

impl ThisParser for UnionFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, (mods, id)) = parse_modifiers(input)?;
        let (state, _) = str(":")(state.skip(ignore))?;
        let (state, typing) = TypingExpression::parse(state.skip(ignore))?;
        state.finish(UnionFieldDeclaration {
            document: Default::default(),
            modifiers: mods,
            field_name: id,
            r#type: typing.as_normal(),
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

fn union_statements(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_into())
        .or_else(|s| UnionFieldDeclaration::parse(s).map_into())
        .or_else(|s| AnnotationList::parse(s).map_into())
        .or_else(|s| AnnotationNode::parse(s).map_into())
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}
