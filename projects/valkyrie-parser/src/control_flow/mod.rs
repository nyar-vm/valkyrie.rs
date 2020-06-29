use crate::{
    helpers::{ignore, parse_eos},
    ThisParser,
};
use lispify::Lisp;
use valkyrie_ast::{ExpressionNode, ExpressionType, FunctionStatementNode, LoopStatementNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

impl ThisParser for LoopStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("loop")?;
        let (state, _) = state.skip(ignore).match_str("{")?;
        let (state, stmts) = state.match_repeats(FunctionStatementNode::parse)?;
        let (state, _) = state.skip(ignore).match_str("}")?;
        let (finally, eos) = state.skip(ignore).match_fn(parse_eos)?;
        finally.finish(LoopStatementNode { body: stmts, eos, range: finally.away_from(finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(self.body.len() + 1);
        terms.push(Lisp::function("loop"));
        for term in &self.body {
            terms.push(term.as_lisp());
        }
        Lisp::Any(terms)
    }
}

impl ThisParser for FunctionStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .skip(ignore)
            .begin_choice()
            .or_else(|s| LoopStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ExpressionNode::parse(s).map_inner(Into::into))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            FunctionStatementNode::Expression(v) => v.as_lisp(),
            FunctionStatementNode::Loop(v) => v.as_lisp(),
        }
    }
}
