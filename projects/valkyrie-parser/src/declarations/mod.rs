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
    ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, ClassFieldDeclaration, ClassMethodDeclaration,
    DocumentationNode, EnumerateDeclaration, EnumerateFieldDeclaration, ExpressionContext, ExpressionNode, FlagsDeclaration,
    FunctionDeclaration, FunctionReturnNode, FunctionType, GenericArgumentNode, IdentifierNode,  ModifiersNode,
    NamePathNode, PrettyPrint, StatementBlock, StatementNode, TaggedDeclaration, TypingExpression, UnionDeclaration,
    VariantDeclaration,
};

fn enum_statements(input: ParseState) -> ParseResult<StatementNode> {
    let (state, ty) = input
        .skip(ignore)
        .begin_choice()
        .or_else(|s| DocumentationNode::parse(s).map_inner(Into::into))
        .or_else(|s| EnumerateFieldDeclaration::parse(s).map_inner(Into::into))
        .end_choice()?;
    state.finish(StatementNode { r#type: ty, end_semicolon: true, span: get_span(input, state) })
}
