use crate::{helpers::ignore, ThisParser};
use lispify::Lisp;
use std::ops::Range;
use valkyrie_ast::{ConditionType, ExpressionNode, ForLoopNode, PatternType, StatementNode, WhileLoopNode};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

pub(crate) struct FunctionBody {
    pub body: Vec<StatementNode>,
}

pub(crate) struct ElseBody {
    pub body: Vec<StatementNode>,
}

impl ThisParser for WhileLoopNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("while")?;
        let (state, condition) = state.skip(ignore).match_fn(ConditionType::parse)?;
        let (state, stmts) = state.skip(ignore).match_fn(FunctionBody::parse)?;
        let (finally, rest) = state.skip(ignore).match_optional(ElseBody::parse)?;
        finally.finish(WhileLoopNode {
            condition,
            body: stmts.body,
            r#else: rest.map(|r#else| r#else.body).unwrap_or_default(),
            range: finally.away_from(input),
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
        state.finish(ForLoopNode {
            pattern: PatternType::Case,
            condition: ConditionType::AlwaysTrue,
            body: vec![],
            r#else: vec![],
            range: state.away_from(input),
        })
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

impl ThisParser for FunctionBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("{")?;
        let (state, stmts) = state.match_repeats(StatementNode::parse)?;
        let (finally, _) = state.skip(ignore).match_str("}")?;
        finally.finish(FunctionBody { body: stmts })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for ElseBody {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(FunctionBody::parse)?;
        state.finish(ElseBody { body: func.body })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}
