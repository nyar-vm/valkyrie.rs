use crate::{helpers::ignore, utils::get_span, ThisParser};
use lispify::Lisp;
use std::{borrow::Cow, ops::Range};
use valkyrie_ast::{
    ConditionNode, ConditionType, ElsePart, ExpressionNode, ForLoopNode, FunctionBodyPart, IfStatementNode, PatternType,
    StatementNode, WhileLoopNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

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
        todo!()
    }
}

impl ThisParser for WhileLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("while")?;
        let (state, condition) = state.skip(ignore).match_fn(ConditionType::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(FunctionBodyPart::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElsePart::parse)?;
        finally.finish(WhileLoopNode {
            condition,
            body: stmts.body.to_vec(),
            r#else: rest.map(|v| v.body.to_vec()).unwrap_or_default(),
            span: get_span(input, finally),
        })
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

impl ThisParser for ForLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("for")?;
        state.finish(ForLoopNode {
            pattern: PatternType::Case,
            condition: ConditionType::AlwaysTrue,
            body: vec![],
            r#else: vec![],
            span: get_span(input, state),
        })
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
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
