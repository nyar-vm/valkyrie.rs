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
        let mut lisp = Lisp::new(3 + self.body.terms.len());
        lisp += Lisp::keyword("enumerate");
        lisp += self.namepath.as_lisp();
        if let Some(s) = &self.layout {
            lisp += Lisp::keyword("layout") + s.as_lisp();
        }
        for term in &self.body.terms {
            lisp += term.as_lisp();
        }
        lisp
    }
}

impl ThisParser for EnumerateFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = IdentifierNode::parse(input)?;
        let (state, value) = state.skip(ignore).match_optional(|s| {
            let (state, _) = str("=")(s)?;
            let (state, expr) = parse_expression_node(state.skip(ignore), ExpressionContext::default())?;
            let state = state.skip(ignore).skip(parse_semi);
            state.finish(expr)
        })?;
        state.finish(EnumerateFieldDeclaration { documentation: Default::default(), name, value, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        lisp += Lisp::keyword("flag");
        lisp += self.name.as_lisp();
        if let Some(value) = &self.value {
            lisp += value.as_lisp();
        }
        lisp
    }
}
