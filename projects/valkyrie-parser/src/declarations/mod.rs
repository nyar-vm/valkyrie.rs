mod function;
mod methods;

use crate::{
    expression::ParseTypeExpression, helpers::ignore, looping::FunctionBody, utils::parse_expression_node, ThisParser,
};
use lispify::Lisp;
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentTermNode, ExpressionBody, ExpressionContext, ExpressionNode, ExpressionType, FunctionCommonPart,
    FunctionDeclarationNode, FunctionType, IdentifierNode, ArgumentKeyNode, NamePathNode, StatementNode,
};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState, StopBecause};
