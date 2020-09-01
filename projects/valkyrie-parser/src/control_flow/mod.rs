use crate::{
    helpers::{ignore, parse_comma, parse_in},
    utils::{get_span, parse_expression_node, parse_modifiers_lookahead},
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPattern, ParseResult, ParseState, StopBecause};

use crate::{helpers::parse_when, utils::parse_statement_block};
use valkyrie_ast::{
    ArgumentKeyNode, ConditionNode, ConditionType, ControlNode, ControlType, ElsePart, ExpressionContext, ExpressionNode,
    ForLoop, IfStatement, PatternType, StatementBlock, StatementNode, WhileLoop,
};

mod controller;
mod loop_for;
mod loop_while;

impl ThisParser for IfStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = parse_when(input)?;
        todo!()
    }

    fn as_lisp(&self) -> Lisp {
        todo!()
    }
}

impl ThisParser for ConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = input.match_fn(ConditionType::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ConditionNode { condition: cond, body, span: get_span(input, state) })
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

impl ThisParser for StatementBlock {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_statement_block(input, StatementNode::parse)
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("body"), Lisp::Any(self.statements.iter().map(|s| s.as_lisp()).collect())])
    }
}

impl ThisParser for ElsePart {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ElsePart { statements: func.statements, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("else"), Lisp::Any(self.statements.iter().map(|s| s.as_lisp()).collect())])
    }
}
