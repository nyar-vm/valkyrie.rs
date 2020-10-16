use crate::{
    expression::{ExpressionResolver, ExpressionStream},
    helpers::ignore,
    ThisParser,
};

use pex::{ParseResult, ParseState, StopBecause};
use std::ops::Range;
use valkyrie_ast::{
    ExpressionContext, ExpressionNode, ExpressionType, IdentifierNode, ModifiersNode, StatementBlock, StatementNode,
};

#[inline]
pub fn get_span(input: ParseState, output: ParseState) -> Range<u32> {
    let range = output.away_from(input);
    (range.start as u32)..(range.end as u32)
}

/// **Warning**: you need to handle the head ignore yourself
pub fn parse_statement_block<F>(input: ParseState, parser: F) -> ParseResult<StatementBlock>
where
    F: Fn(ParseState) -> ParseResult<StatementNode>,
{
    let (state, _) = input.match_str("{")?;
    let (state, stmts) = state.match_repeats(parser)?;
    let (finally, _) = state.skip(ignore).match_str("}")?;
    finally.finish(StatementBlock { terms: stmts, span: get_span(input, state) })
}

pub fn parse_expression_node(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionNode> {
    let resolver = ExpressionResolver::default();
    let (state, stream) = ExpressionStream::parse(input, config)?;
    let body = resolver.resolve(stream)?;
    state.finish(ExpressionNode { type_level: config.type_level, body, span: get_span(input, state) })
}

pub fn parse_expression_body(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionType> {
    parse_expression_node(input, config).map_inner(|s| s.body)
}

pub fn parse_modifiers(input: ParseState) -> ParseResult<(ModifiersNode, IdentifierNode)> {
    parse_modifiers_lookahead(input, |_| false)
}

pub fn parse_modifiers_lookahead<F>(input: ParseState, negative: F) -> ParseResult<(ModifiersNode, IdentifierNode)>
where
    F: Fn(&str) -> bool + Copy,
{
    let (finally, mut ids) = input.match_repeats(|s| {
        let (state, id) = s.skip(ignore).match_fn(IdentifierNode::parse)?;
        if negative(&id.name) {
            StopBecause::custom_error("Unexpected modifier", state.start_offset, state.start_offset)?
        }
        state.finish(id)
    })?;
    match ids.pop() {
        Some(id) => finally.finish((ModifiersNode::new(ids), id)),
        None => StopBecause::custom_error("Expected at least one modifier", finally.start_offset, finally.start_offset)?,
    }
}
