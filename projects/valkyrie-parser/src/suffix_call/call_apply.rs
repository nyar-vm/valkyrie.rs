use super::*;

impl<E> ThisParser for GenericArgumentNode<E> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ApplyCallNode<ExpressionBody> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyTermNode::parse)?;
        state.finish(ApplyCallNode { base: ExpressionBody::Placeholder, terms: terms.body, range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 2);
        terms.push(Lisp::keyword("apply"));
        terms.push(self.base.as_lisp());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}
