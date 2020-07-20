use crate::expression::{ExpressionResolver, ExpressionStream};
use valkyrie_ast::{ExpressionBody, ExpressionContext, ExpressionNode};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

pub fn parse_expression_node(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionNode> {
    let resolver = ExpressionResolver::default();
    let (state, stream) = ExpressionStream::parse(input, config)?;
    let body = resolver.resolve(stream)?;
    state.finish(ExpressionNode { r#type: config.as_type(), body, range: state.away_from(input) })
}

pub fn parse_expression_body(input: ParseState, config: ExpressionContext) -> ParseResult<ExpressionBody> {
    parse_expression_node(input, config).map_inner(|s| s.body)
}
