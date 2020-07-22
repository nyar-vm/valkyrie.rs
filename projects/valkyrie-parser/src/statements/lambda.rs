use super::*;
use valkyrie_ast::{LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode};

impl ThisParser for LambdaNode {
    /// `{ body }`
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("{")?;
        let (state, arguments) = state.skip(ignore).match_optional(LambdaArgumentNode::parse)?;
        let (state, body) = state.skip(ignore).match_repeats(StatementNode::parse)?;
        let (finally, _) = state.skip(ignore).match_str("}")?;
        finally.finish(LambdaNode { arguments, body, range: finally.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for LambdaArgumentNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("lambda")?;
        state.finish(LambdaArgumentNode { terms: vec![], range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for LambdaCallNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        LambdaNode::parse(input).map_inner(|s| s.as_lambda_call())
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.len() + 2);
        terms.push(Lisp::keyword("call/lambda"));
        terms.push(self.base.as_lisp());
        terms.extend(self.body.iter().map(ThisParser::as_lisp));
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
        terms.push(Lisp::keyword("call/lambda-dot"));
        terms.push(self.base.as_lisp());
        terms.extend(self.body.iter().map(ThisParser::as_lisp));
        Lisp::Any(terms)
    }
}
