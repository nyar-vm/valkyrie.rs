use crate::{
    helpers::{ignore, parse_in, parse_when},
    utils::{get_span, parse_expression_node, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    CallNode, ControlNode, ControlType, ElseStatement, ExpressionContext, ExpressionNode, ForLoop, GuardLetStatement,
    GuardStatement, GuardStatementBody, IfConditionNode, IfLetStatement, IfStatement, MatchDotStatement, MatchKind,
    MonadicDotCall, OtherwiseStatement, PatternBlock, PatternExpressionType, PatternGuard, StatementBlock, StatementNode,
    SwitchStatement, ThenStatement, WhileConditionNode, WhileLoop, WhileLoopKind,
};

mod controller;
mod jmp_guard;
mod jmp_if;
mod jmp_match;
mod jmp_switch;
mod loop_for;
mod loop_while;

impl ThisParser for WhileConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose(|s| {
                let (state, e) = ExpressionNode::parse(s)?;
                state.finish(WhileConditionNode::Expression(Box::new(e)))
            })
            .choose(|s| s.finish(WhileConditionNode::Unconditional))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            WhileConditionNode::Unconditional => Lisp::keyword("true"),
            WhileConditionNode::Case => Lisp::keyword("case"),
            WhileConditionNode::Expression(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for StatementBlock {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_statement_block(input, StatementNode::parse)
    }

    fn as_lisp(&self) -> Lisp {
        let mut lisp = Lisp::new(self.terms.len());
        for term in self.terms.iter() {
            lisp += term.as_lisp();
        }
        lisp
    }
}
pub fn parse_maybe_then(input: ParseState) -> ParseResult<ThenStatement> {
    let (state, _) = input.match_optional(|s| s.match_str("then"))?;
    let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
    state.finish(ThenStatement { show: true, body, span: get_span(input, state) })
}
