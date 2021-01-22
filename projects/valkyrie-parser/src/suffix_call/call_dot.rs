use super::*;

impl ThisParser for CallNode<ApplyDotNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        lisp += Lisp::keyword("call/apply-dot");
        lisp += self.base.lispify();
        lisp += self.rest.lispify();
        lisp
    }
}

impl ThisParser for ApplyDotNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        let (state, caller) = state.skip(ignore).match_fn(NamePathNode::parse)?;
        let (finally, args) = state.skip(ignore).match_optional(ApplyCallNode::parse)?;
        let terms = match args {
            Some(v) => v.arguments,
            None => vec![],
        };
        finally.finish(ApplyDotNode { nullable: false, caller, terms, span: get_span(input, finally) })
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len() + 3);
        lisp += self.caller.lispify();
        for term in &self.terms {
            lisp += term.lispify();
        }
        lisp
    }
}
