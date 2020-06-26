use super::*;

impl<E> ThisParser for GenericArgumentNode<E> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ApplyCallNode<TermExpressionNode> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyTermNode::parse)?;
        state.finish(ApplyCallNode { base: TermExpressionNode::Placeholder, terms: terms.body, range: state.away_from(input) })
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

impl ThisParser for ApplyDotNode<TermExpressionNode> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        let (state, caller) = state.skip(ignore).match_fn(IdentifierNode::parse)?;
        let (finally, args) = state.skip(ignore).match_optional(ApplyCallNode::parse)?;
        let terms = match args {
            Some(v) => v.terms,
            None => vec![],
        };
        finally.finish(ApplyDotNode { base: TermExpressionNode::Placeholder, caller, terms, range: finally.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.terms.len() + 3);
        terms.push(Lisp::keyword("apply-dot"));
        terms.push(self.base.as_lisp());
        terms.push(self.caller.as_lisp());
        for term in &self.terms {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}
