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
        todo!()
        // let mut terms = Vec::with_capacity(5);
        // terms.push(Lisp::keyword("guard"));
        // terms.push(self.condition.as_lisp());
        // terms.push(Lisp::keyword("else"));
        // terms.push(self.body.as_lisp());
        // lisp
    }
}

impl ThisParser for GuardPattern {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| ImplicitCaseNode::parse(s).map_into())
            .or_else(|s| ExpressionNode::parse(s).map_into())
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            GuardPattern::Case(s) => s.as_lisp(),
            GuardPattern::Inline(s) => s.as_lisp(),
        }
    }
}
