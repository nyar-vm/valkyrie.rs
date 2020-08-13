mod function;
mod methods;

use crate::{
    expression::TypingExpression,
    helpers::ignore,
    utils::{get_span, parse_expression_node, parse_modifiers},
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPattern, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, ExpressionBody, ExpressionContext, ExpressionNode, FunctionBodyPart,
    FunctionCommonPart, FunctionDeclaration, FunctionType, GenericArgumentNode, IdentifierNode, ModifierPart, NamePathNode,
    PrettyPrint, StatementNode,
};
