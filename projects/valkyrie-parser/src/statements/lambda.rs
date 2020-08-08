use super::*;

impl ThisParser for LambdaNode {
    /// `{ body }`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("{")?;
        let (state, arguments) = state.skip(ignore).match_optional(LambdaArgumentNode::parse)?;
        let (state, body) = state.skip(ignore).match_repeats(StatementNode::parse)?;
        let (finally, _) = state.skip(ignore).match_str("}")?;
        finally.finish(LambdaNode { arguments, body, span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for LambdaArgumentNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("lambda")?;
        state.finish(LambdaArgumentNode { terms: vec![], range: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
