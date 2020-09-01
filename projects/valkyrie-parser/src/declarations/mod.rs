mod flags;
mod function;
mod methods;
use crate::{
    helpers::{ignore, parse_semi, parse_when},
    statements::parse_statement_node,
    utils::{get_span, parse_expression_node, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{
    helpers::{char, str},
    BracketPattern, ParseResult, ParseState, StopBecause,
};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, DocumentationNode, ExpressionContext,
    FlagFieldDeclaration, FlagsDeclaration, FunctionDeclaration, FunctionReturnNode, FunctionType, GenericArgumentNode,
    IdentifierNode, NamePathNode, PrettyPrint, StatementBlock, StatementNode,
};
