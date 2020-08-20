mod function;
mod methods;

use crate::{
    helpers::ignore,
    utils::{get_span, parse_expression_node},
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPattern, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, ExpressionContext, ExpressionNode, FunctionBody, FunctionDeclaration,
    FunctionType, GenericArgumentNode, IdentifierNode, NamePathNode, PrettyPrint,
};
