use super::*;

impl ThisParser for CallNode<ApplyCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(3);
        lisp += Lisp::keyword("call/argument");
        lisp += self.base.lispify();
        lisp += self.rest.lispify();
        lisp
    }
}

impl ThisParser for ApplyCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        let (state, terms) = pat.consume(input, ignore, ApplyCallItem::parse)?;
        state.finish(ApplyCallNode { arguments: terms.body, span: get_span(input, state) })
    }
}

impl ThisParser for ApplyCallItem {
    fn parse(input: ParseState) -> ParseResult<Self> {
        CallTermNode::parse(input).map_inner(|term| ApplyCallItem { term })
    }
}
