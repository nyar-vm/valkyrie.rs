use super::*;
use crate::helpers::parse_colon;
use valkyrie_ast::StatementNode;

impl ThisParser for FlagsDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = str("flags")(input)?;
        let (state, id) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let pat = BracketPattern::new("{", "}");
        let (state, terms) = pat.consume(state.skip(ignore), ignore, parse_statement)?;

        state.finish(FlagsDeclaration {
            documentation: Default::default(),
            namepath: id,
            modifiers: vec![],
            layout: None,
            implements: vec![],
            statements: terms.body,
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(2 + self.statements.len());
        terms.push(Lisp::keyword("flags"));
        terms.push(self.namepath.as_lisp());
        for term in &self.statements {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for FlagFieldDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, name) = input.skip(ignore).match_fn(IdentifierNode::parse)?;
        let (state, value) = state.skip(ignore).match_optional(|s| {
            let (state, expr) = parse_expression_node(
                s.skip(str("=")).skip(ignore),
                ExpressionContext { type_level: false, allow_newline: true, allow_curly: false },
            )?;
            let (state, _) = state.skip(ignore).match_optional(parse_colon)?;
            state.finish(expr)
        })?;
        state.finish(FlagFieldDeclaration { documentation: Default::default(), name, value, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(2);
        terms.push(self.name.as_lisp());
        if let Some(value) = &self.value {
            terms.push(value.as_lisp());
        }
        Lisp::Any(terms)
    }
}

fn parse_statement(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input.begin_choice().or_else(|s| FlagFieldDeclaration::parse(s).map_inner(Into::into)).end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}
