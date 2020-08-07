use super::*;
use valkyrie_ast::{ApplyCallTerm, ArgumentTermNode, CallNode, ExpressionNode};

impl ThisParser for CallNode<ApplyCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("call/apply"));
        terms.push(self.base.as_lisp());
        terms.push(self.rest.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for ApplyCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyCallTerm::parse)?;
        state.finish(ApplyCallNode { terms: terms.body, range: state.away_from(input) })
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

impl ThisParser for ApplyCallTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        CallTermNode::parse(input).map_inner(|term| ApplyCallTerm { term })
    }

    fn as_lisp(&self) -> Lisp {
        self.term.as_lisp()
    }
}
