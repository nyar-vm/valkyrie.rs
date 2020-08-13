use crate::{helpers::ignore, utils::get_span, ThisParser};
use lispify::Lisp;
use pex::{ParseResult, ParseState};
use std::{borrow::Cow, ops::Range};
use valkyrie_ast::{
    ConditionNode, ConditionType, ControlNode, ControlType, ElsePart, ExpressionNode, ForLoopNode, FunctionBodyPart,
    IfStatementNode, PatternType, StatementNode, WhileLoopNode,
};

mod controller;
mod loop_for;
mod loop_while;

impl ThisParser for IfStatementNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = input.match_fn(ConditionType::parse)?;
        let (state, body) = state.skip(ignore).match_fn(FunctionBodyPart::parse)?;
        state.finish(ConditionNode { condition: cond, body: body.body.to_vec(), span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
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
        match self {
            ConditionType::AlwaysTrue => Lisp::keyword("true"),
            ConditionType::Case => Lisp::keyword("case"),
            ConditionType::Expression(v) => v.as_lisp(),
        }
    }
}

impl<'i> ThisParser for FunctionBodyPart<'i> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("{")?;
        let (state, stmts) = state.match_repeats(StatementNode::parse)?;
        let (finally, _) = state.skip(ignore).match_str("}")?;
        finally.finish(FunctionBodyPart { body: Cow::Owned(stmts) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl<'i> ThisParser for ElsePart<'i> {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(FunctionBodyPart::parse)?;
        state.finish(ElsePart { body: func.body })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}
