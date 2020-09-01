mod flags;
mod function;
mod methods;
mod unions;

use crate::{
    helpers::{ignore, parse_semi},
    utils::{get_span, parse_expression_node, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, BracketPattern, ParseResult, ParseState, StopBecause};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyArgumentTerm, ArgumentKeyNode, ArgumentTermNode, DocumentationNode, ExpressionContext,
    FieldDeclaration, FlagFieldDeclaration, FlagsDeclaration, FunctionDeclaration, FunctionReturnNode, FunctionType,
    GenericArgumentNode, IdentifierNode, ModifiersNode, NamePathNode, PrettyPrint, StatementBlock, StatementNode,
    UnionDeclaration, VariantDeclaration,
};
