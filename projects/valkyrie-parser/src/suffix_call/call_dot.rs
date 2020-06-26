use super::*;

impl ThisParser for GenericCall<TermExpressionNode> {
    /// `::<T> | ⦓T⦔`
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().or_else(qwerty_generic).or_else(unicode_generic).end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::function("generic"));
        // terms.push(self.base.lispify().into());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

fn qwerty_generic(input: ParseState) -> ParseResult<GenericCall<TermExpressionNode>> {
    let pat = BracketPattern::new("<", ">");
    let (state, _) = input.match_optional(parse_name_join)?;
    let (state, terms) = pat.consume(state.skip(ignore), ignore, ApplyTermNode::parse)?;
    state.finish(GenericCall { base: TermExpressionNode::Placeholder, terms: terms.body, range: state.away_from(input) })
}

fn unicode_generic(input: ParseState) -> ParseResult<GenericCall<TermExpressionNode>> {
    let pat = BracketPattern::new("⦓", "⦔");
    let (state, terms) = pat.consume(input, ignore, ApplyTermNode::parse)?;
    state.finish(GenericCall { base: TermExpressionNode::Placeholder, terms: terms.body, range: state.away_from(input) })
}
