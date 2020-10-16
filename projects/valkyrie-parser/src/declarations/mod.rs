mod classes;
mod enumerate;
mod flags;
mod function;
mod macros;
mod tagged;
mod unions;

use crate::{
    helpers::{ignore, parse_semi},
    utils::{get_span, parse_expression_node, parse_modifiers, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, BracketPattern, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    AnnotationList, AnnotationNode, ApplyArgument, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, ClassDeclaration,
    ClassFieldDeclaration, ClassKind, ClassMethodDeclaration, DocumentationNode, EnumerateDeclaration,
    EnumerateFieldDeclaration, ExpressionContext, ExpressionNode, FlagsDeclaration, FunctionDeclaration, FunctionEffectNode,
    FunctionReturnNode, FunctionType, GenericArgument, IdentifierNode, ModifiersNode, NamePathNode, StatementBlock,
    StatementNode, TaggedDeclaration, TypingExpression, UnionDeclaration, UnionFieldDeclaration, VariantDeclaration,
};
fn enum_statements(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .choose(|s| DocumentationNode::parse(s).map_into())
        .choose(|s| EnumerateFieldDeclaration::parse(s).map_into())
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}
