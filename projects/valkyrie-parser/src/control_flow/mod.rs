use crate::{
    helpers::{ignore, parse_in, parse_when},
    utils::{get_span, parse_expression_node, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, ParseResult, ParseState};
use valkyrie_ast::{
    ControlNode, ControlType, ElseStatement, ExpressionContext, ExpressionNode, ForLoop, IfConditionNode, IfStatement,
    PatternExpression, PatternGuard, StatementBlock, StatementNode, WhileConditionNode, WhileLoop,
};

mod controller;
mod loop_for;
mod loop_while;

impl ThisParser for IfStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = parse_when(input)?;
        let (state, cond) = state.skip(ignore).match_fn(IfConditionNode::parse)?;
        let (state, body) = state.match_repeats(parse_else_if)?;
        let (finally, else_body) = state.skip(ignore).match_optional(ElseStatement::parse)?;
        let mut branches = vec![cond];
        branches.extend(body);
        finally.finish(IfStatement { branches, else_branch: else_body.unwrap_or_default(), span: get_span(input, finally) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(Lisp::keyword("branches"));
        for branch in &self.branches {
            terms.push(branch.as_lisp());
        }
        terms.push(self.else_branch.as_lisp());
        Lisp::Any(terms)
    }
}

fn parse_else_if(input: ParseState) -> ParseResult<IfConditionNode> {
    let (state, _) = str("else")(input.skip(ignore))?;
    let (state, _) = str("if")(state.skip(ignore))?;
    let (state, cond) = state.skip(ignore).match_fn(IfConditionNode::parse)?;
    state.finish(cond)
}

impl ThisParser for IfConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, cond) = input.match_fn(ExpressionNode::parse)?;
        let (state, body) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(IfConditionNode { condition: cond, body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        let mut terms = Vec::with_capacity(10);
        terms.push(self.condition.as_lisp());
        terms.push(self.body.as_lisp());
        Lisp::Any(terms)
    }
}

impl ThisParser for WhileConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| {
                let (state, e) = ExpressionNode::parse(s)?;
                state.finish(WhileConditionNode::Expression(Box::new(e)))
            })
            .or_else(|s| s.finish(WhileConditionNode::AlwaysTrue))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            WhileConditionNode::AlwaysTrue => Lisp::keyword("true"),
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
        Lisp::Any(vec![Lisp::keyword("body"), Lisp::Any(self.terms.iter().map(|s| s.as_lisp()).collect())])
    }
}

impl ThisParser for ElseStatement {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("else")?;
        let (state, func) = state.skip(ignore).match_fn(StatementBlock::parse)?;
        state.finish(ElseStatement { statements: func.terms, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("else"), Lisp::Any(self.statements.iter().map(|s| s.as_lisp()).collect())])
    }
}
