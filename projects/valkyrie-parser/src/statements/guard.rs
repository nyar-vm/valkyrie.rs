use super::*;

impl ThisParser for GuardStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("guard")?;
        let (state, cond) = state.skip(ignore).match_fn(GuardPattern::parse)?;
        let (state, _) = state.skip(ignore).match_str("else")?;
        let (finally, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
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

impl ThisParser for GuardPattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, node) = parse_expression_node(input.skip(ignore), ExpressionContext::default())?;
        state.finish(GuardPattern::Inline(Box::new(node)))
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            GuardPattern::Case => Lisp::keyword("case"),
            GuardPattern::Inline(s) => s.as_lisp(),
            // GuardType::Block(s) => s.as_lisp(),
        }
    }
}