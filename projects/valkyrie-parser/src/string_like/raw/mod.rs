use super::*;

impl ThisParser for StringLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, unit) = input.match_optional(IdentifierNode::parse)?;
        let (state, pair) = state
            .begin_choice()
            .choose(|s| quotation_pair_nested(s, '\''))
            .choose(|s| quotation_pair_nested(s, '"'))
            .choose(|s| quotation_pair(s, '«', '»'))
            .end_choice()?;

        state.finish(StringLiteralNode { literal: pair.body.as_string(), handler: unit, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let literal = Lisp::string(self.literal.to_string());
        match &self.handler {
            Some(s) => Lisp::unit(s.name.clone()) & literal,
            None => literal,
        }
    }
}
