use super::*;

impl ThisParser for FlagsDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("flags")(input)?;
        let (state, id) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (state, stmt) = parse_statement_block(state.skip(ignore), enum_statements)?;

        state.finish(FlagsDeclaration {
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
        terms.push(Lisp::keyword("flags"));
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
