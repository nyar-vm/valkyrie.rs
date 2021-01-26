use super::*;

impl ThisParser for LambdaNode {
    /// `{ body }`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("{")?;
        let (state, arguments) = state.skip(ignore).match_optional(FunctionBlock::parse)?;
        let (state, body) = state.skip(ignore).match_repeats(StatementNode::parse)?;
        let (finally, _) = state.skip(ignore).match_str("}")?;
        finally.finish(LambdaNode { arguments, body, span: get_span(input, finally) })
    }
}

impl ThisParser for FunctionBlock {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("lambda")?;
        state.finish(FunctionBlock { terms: vec![], range: get_span(input, state) })
    }
}
