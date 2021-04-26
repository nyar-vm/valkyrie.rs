use super::*;

impl ThisParser for CallNode<ClosureCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("call/lambda") + self.base.lispify() + self.rest.lispify()
    }
}

impl ThisParser for ClosureCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        LambdaNode::parse(input).map_inner(|s| s.as_lambda_call())
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.caller.len() + 2);
        for term in &self.caller {
            lisp += term.lispify();
        }
        lisp
    }
}

impl ThisParser for CallNode<LambdaDotNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn lispify(&self) -> Lisp {
        Lisp::keyword("call/lambda-dot") + self.base.lispify() + self.rest.lispify()
    }
}

impl ThisParser for LambdaDotNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str(".")?;
        state.skip(ignore).match_fn(LambdaNode::parse).map_inner(|s| s.as_lambda_dot())
    }

    fn lispify(&self) -> Lisp {
        let mut lisp = Lisp::new(self.body.len() + 2);
        for term in &self.body {
            lisp += term.lispify();
        }
        lisp
    }
}
