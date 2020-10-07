use super::*;

impl ThisParser for StringLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, unit) = input.match_optional(IdentifierNode::parse)?;
        let (state, pair) = state
            .begin_choice()
            .or_else(|s| quotation_pair_nested(s, '\''))
            .or_else(|s| quotation_pair_nested(s, '"'))
            .or_else(|s| quotation_pair(s, '«', '»'))
            .end_choice()?;

        state.finish(StringLiteralNode { raw: pair.body.as_string(), unit, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let literal = Lisp::string(self.raw.to_string());
        match &self.unit {
            Some(s) => Lisp::unit(s.name.clone()) & literal,
            None => literal,
        }
    }
}
