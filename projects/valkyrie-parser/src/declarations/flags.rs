use super::*;
use valkyrie_ast::DocumentationNode;

impl ThisParser for FlagsDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("flags")(input)?;
        let (state, id) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let pat = BracketPattern::new("{", "}");
        let (state, terms) = pat.consume(state.skip(ignore), ignore, flags_statement)?;
        let stmt = StatementBlock { statements: terms.body, span: get_span(input, state) };

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
        let mut terms = Vec::with_capacity(3 + self.body.statements.len());
        terms.push(Lisp::keyword("flags"));
        terms.push(self.namepath.as_lisp());
        if let Some(s) = &self.layout {
            terms.push(Lisp::Any(vec![Lisp::keyword("layout"), s.as_lisp()]))
        }
        for term in &self.body.statements {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for FlagFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = input.match_fn(IdentifierNode::parse)?;
        let (state, value) = state.skip(ignore).match_optional(|s| {
            let (state, _) = str("=")(s)?;
            let (state, expr) = parse_expression_node(
                state.skip(ignore),
                ExpressionContext { type_level: false, allow_newline: true, allow_curly: false },
            )?;
            let (state, _) = state.skip(ignore).match_optional(parse_semi)?;
            state.finish(expr)
        })?;
        state.finish(FlagFieldDeclaration { documentation: Default::default(), name, value, span: get_span(input, state) })
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

fn flags_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_inner(Into::into))
        .or_else(|s| FlagFieldDeclaration::parse(s).map_inner(Into::into))
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}
