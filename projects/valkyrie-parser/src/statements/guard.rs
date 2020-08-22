use super::*;

impl ThisParser for GuardStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, cond) = state.skip(ignore).match_fn(GuardType::parse)?;
        let (state, _) = state.skip(ignore).match_str("else")?;
        let (finally, body) = state.skip(ignore).match_fn(FunctionBody::parse)?;
        finally.finish(GuardStatement { condition: cond, body, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(5);
        terms.push(Lisp::keyword("guard"));
        terms.push(self.condition.as_lisp());
        terms.push(Lisp::keyword("else"));
        terms.push(self.body.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for GuardType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, node) = parse_expression_node(
            input.skip(ignore),
            ExpressionContext { type_level: false, allow_newline: true, allow_curly: false },
        )?;
        state.finish(GuardType::Inline(Box::new(node)))
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            GuardType::Case => Lisp::keyword("case"),
            GuardType::Inline(s) => s.as_lisp(),
            // GuardType::Block(s) => s.as_lisp(),
        }
    }
}
