use super::*;

impl ThisParser for CallNode<ApplyCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        lisp += Lisp::keyword("call/apply");
        lisp += self.base.lispify();
        lisp += self.rest.lispify();
        lisp
    }
}

impl ThisParser for ApplyCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyCallTerm::parse)?;
        state.finish(ApplyCallNode { terms: terms.body, span: get_span(input, state) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 2);
        lisp += Lisp::keyword("apply");
        for term in &self.terms {
            lisp += term.lispify();
        }
        lisp
    }
}

impl ThisParser for ApplyCallTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        CallTermNode::parse(input).map_inner(|term| ApplyCallTerm { term })
    }

    fn lispify(&self) -> Lisp {
        self.term.lispify()
    }
}
