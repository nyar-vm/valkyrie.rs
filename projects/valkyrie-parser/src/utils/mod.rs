use crate::{
    expression::{ExpressionResolver, ExpressionStream},
    helpers::{ignore, parse_eos},
    ThisParser,
};
use lispify::Lisp;
use std::ops::Range;
use valkyrie_ast::{
    ClassDeclaration, ExpressionBody, ExpressionContext, ExpressionNode, ForLoopNode, FunctionCommonPart, FunctionDeclaration,
    FunctionType, IdentifierNode, ImportStatementNode, ModifierPart, NamePathNode, NamespaceDeclarationNode, StatementContext,
    StatementNode, StatementType, WhileLoopNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState, StopBecause};

#[inline]
pub fn get_span(input: ParseState, output: ParseState) -> Range<u32> {
    let range = output.away_from(input);
    (range.start as u32)..(range.end as u32)
}

pub fn parse_expression_node(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionNode> {
    let resolver = ExpressionResolver::default();
    let (state, stream) = ExpressionStream::parse(input, config)?;
    let body = resolver.resolve(stream)?;
    state.finish(ExpressionNode { type_level: config.type_level, body, span: get_span(input, state) })
}

pub fn parse_expression_body(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionBody> {
    parse_expression_node(input, config).map_inner(|s| s.body)
}

pub fn parse_modifiers<F, T>(input: ParseState, negative: F) -> ParseResult<(Vec<IdentifierNode>, IdentifierNode)>
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
        Some(id) => finally.finish((ids, id)),
        None => StopBecause::custom_error("Expected at least one modifier", finally.start_offset, finally.start_offset)?,
    }
}
