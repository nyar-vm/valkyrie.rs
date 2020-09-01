mod flags;
mod function;
mod methods;
use crate::{
    helpers::{ignore, parse_semi, parse_when},
    utils::{get_span, parse_expression_node},
    ThisParser,
};
use lispify::Lisp;
use pex::{
    helpers::{char, str},
    BracketPattern, ParseResult, ParseState, StopBecause,
};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, ExpressionContext, FlagFieldDeclaration,
    FlagsDeclaration, FunctionDeclaration, FunctionReturnNode, FunctionType, GenericArgumentNode, IdentifierNode, NamePathNode,
    PrettyPrint, StatementBlock, StatementNode,
};
