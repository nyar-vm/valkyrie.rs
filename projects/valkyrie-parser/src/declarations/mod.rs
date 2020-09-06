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
    DocumentationNode, EnumerateDeclaration, ExpressionContext, ExpressionNode, FlagsDeclaration, FlagsFieldDeclaration,
    FunctionDeclaration, FunctionReturnNode, FunctionType, GenericArgumentNode, IdentifierNode, MacroPathNode, ModifiersNode,
    NamePathNode, PrettyPrint, StatementBlock, StatementNode, TaggedDeclaration, TypingExpression, UnionDeclaration,
    VariantDeclaration,
};
