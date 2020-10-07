use super::*;

impl ThisParser for CallNode<LambdaCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("call/lambda") + self.base.as_lisp() + self.rest.as_lisp()
    }
}

impl ThisParser for LambdaCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        LambdaNode::parse(input).map_inner(|s| s.as_lambda_call())
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.body.len() + 2);
        for term in &self.body {
            lisp += term.as_lisp();
        }
        lisp
    }
}

impl ThisParser for CallNode<LambdaDotNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::keyword("call/lambda-dot") + self.base.as_lisp() + self.rest.as_lisp()
    }
}

impl ThisParser for LambdaDotNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str(".")?;
        state.skip(ignore).match_fn(LambdaNode::parse).map_inner(|s| s.as_lambda_dot())
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.body.len() + 2);
        for term in &self.body {
            lisp += term.as_lisp();
        }
        lisp
    }
}
