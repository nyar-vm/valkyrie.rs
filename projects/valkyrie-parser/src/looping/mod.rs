use crate::{
    helpers::{ignore, parse_eos},
    ThisParser,
};
use lispify::Lisp;
use std::ops::Range;
use valkyrie_ast::{ConditionType, ElseNode, ExpressionNode, ExpressionType, ForLoopNode, StatementNode, WhileLoopNode};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState};

impl ThisParser for WhileLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("while")?;
        let (state, condition) = state.skip(ignore).match_fn(ConditionType::parse)?;
        let (finally, stmts) = state.skip(ignore).match_fn(parse_function_body)?;

        finally.finish(WhileLoopNode { condition, body: stmts, r#else: vec![], range: finally.away_from(input) })
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

pub fn parse_function_body(input: ParseState) -> ParseResult<Vec<StatementNode>> {
    let (state, _) = input.match_str("{")?;
    let (state, stmts) = state.match_repeats(StatementNode::parse)?;
    let (finally, _) = state.skip(ignore).match_str("}")?;
    finally.finish(stmts)
}

impl ThisParser for ConditionType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| {
                let (state, e) = ExpressionNode::parse(s)?;
                state.finish(ConditionType::Expression(Box::new(e)))
            })
            .or_else(|s| s.finish(ConditionType::AlwaysTrue))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ForLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        state.finish(ForLoopNode { body: vec![], r#else: vec![], range: state.away_from(input) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

pub struct ForStatementNode<E, B> {
    pub iterator: E,
    pub body: B,
    pub range: Range<usize>,
}

impl ThisParser for ElseNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.skip(ignore).match_str("else")?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
