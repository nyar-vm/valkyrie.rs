use super::*;

impl ThisParser for FlagsDeclaration {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
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
            let (state, _) = state.skip(ignore).match_optional(str(";"))?;
            state.finish(expr)
        })?;
        state.finish(FlagFieldDeclaration { documentation: Default::default(), name, value, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
