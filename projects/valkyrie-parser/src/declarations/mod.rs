mod function;
mod methods;

use crate::{
    expression::ParseTypeExpression,
    helpers::ignore,
    looping::FunctionBody,
    utils::{parse_expression_node, parse_modifiers},
    ThisParser,
};
use lispify::Lisp;
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, ExpressionBody, ExpressionContext, ExpressionNode, ExpressionType,
    FunctionCommonPart, FunctionDeclaration, FunctionType, IdentifierNode, ModifierPart, NamePathNode, PrettyPrint,
    StatementNode,
};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState, StopBecause};
