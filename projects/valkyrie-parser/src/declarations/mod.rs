mod flags;
mod function;
mod methods;
mod unions;

use crate::{
    helpers::{ignore, parse_semi},
    utils::{get_span, parse_expression_node, parse_modifiers, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, BracketPattern, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, ClassFieldDeclaration, DocumentationNode,
    ExpressionContext, ExpressionNode, FlagsDeclaration, FlagsFieldDeclaration, FunctionDeclaration, FunctionReturnNode,
    FunctionType, GenericArgumentNode, IdentifierNode, ModifiersNode, NamePathNode, PrettyPrint, StatementBlock, StatementNode,
    TypingExpression, UnionDeclaration, VariantDeclaration,
};
