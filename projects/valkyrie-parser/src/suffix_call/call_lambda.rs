use super::*;

impl ThisParser for CallNode<LambdaCallNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("call/lambda"));
        terms.push(self.base.as_lisp());
        terms.push(self.rest.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for LambdaCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        LambdaNode::parse(input).map_inner(|s| s.as_lambda_call())
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.len() + 2);
        terms.extend(self.body.iter().map(ThisParser::as_lisp));
        Lisp::Any(terms)
    }
}

impl ThisParser for CallNode<LambdaDotNode> {
    #[track_caller]
    fn parse(_: ParseState) -> ParseResult<Self> {
        unreachable!()
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(3);
        terms.push(Lisp::keyword("call/lambda-dot"));
        terms.push(self.base.as_lisp());
        terms.push(self.rest.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for LambdaDotNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str(".")?;
        state.skip(ignore).match_fn(LambdaNode::parse).map_inner(|s| s.as_lambda_dot())
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.len() + 2);
        terms.extend(self.body.iter().map(ThisParser::as_lisp));
        Lisp::Any(terms)
    }
}
