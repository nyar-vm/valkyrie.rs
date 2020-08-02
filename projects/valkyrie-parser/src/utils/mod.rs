use crate::{
    expression::{ExpressionResolver, ExpressionStream},
    helpers::{ignore, parse_eos},
    ThisParser,
};
use lispify::Lisp;
use valkyrie_ast::{
    ClassDeclaration, ExpressionBody, ExpressionContext, ExpressionNode, ForLoopNode, FunctionCommonPart,
    FunctionDeclaration, FunctionType, IdentifierNode, ImportStatementNode, ModifierPart, NamePathNode,
    NamespaceDeclarationNode, StatementContext, StatementNode, StatementType, WhileLoopNode,
};
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

pub fn parse_modifiers<'i, F, T>(input: ParseState<'i>, negative: F) -> ParseResult<'i, ModifierPart>
where
    F: FnMut(ParseState<'i>) -> ParseResult<'i, T> + Copy,
{
    let (state, out) = input.match_repeats(|s| {
        let (state, _) = s.skip(ignore).match_negative(negative, "MODIFIER")?;
        state.match_fn(IdentifierNode::parse)
    })?;
    state.finish(ModifierPart { modifiers: out })
}
