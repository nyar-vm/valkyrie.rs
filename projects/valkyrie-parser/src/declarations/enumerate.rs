use super::*;

impl ThisParser for EnumerateDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("enumerate")(input)?;
        let (state, id) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), enum_statements)?;

        state.finish(EnumerateDeclaration {
            documentation: Default::default(),
            namepath: id,
            modifiers: vec![],
            layout: None,
            implements: vec![],
            body: stmt,
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3 + self.body.terms.len());
        terms.push(Lisp::keyword("enumerate"));
        terms.push(self.namepath.as_lisp());
        if let Some(s) = &self.layout {
            terms.push(Lisp::Any(vec![Lisp::keyword("layout"), s.as_lisp()]))
        }
        for term in &self.body.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for EnumerateFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = IdentifierNode::parse(input)?;
        let (state, value) = state.skip(ignore).match_optional(|s| {
            let (state, _) = str("=")(s)?;
            let (state, expr) = parse_expression_node(
                state.skip(ignore),
                ExpressionContext { type_level: false, allow_newline: true, allow_curly: false },
            )?;
            let state = state.skip(ignore).skip(parse_semi);
            state.finish(expr)
        })?;
        state.finish(EnumerateFieldDeclaration { documentation: Default::default(), name, value, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("flag"));
        terms.push(self.name.as_lisp());
        if let Some(value) = &self.value {
            terms.push(value.as_lisp());
        }
        Lisp::Any(terms)
    }
}
