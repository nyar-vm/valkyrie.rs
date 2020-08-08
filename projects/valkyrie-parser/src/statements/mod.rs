use crate::{
    helpers::{ignore, parse_eos},
    traits::ThisParser,
    utils::{get_span, parse_expression_node, parse_modifiers},
};
use lispify::Lisp;
use valkyrie_ast::{
    ClassDeclaration, ExpressionContext, ExpressionNode, ForLoopNode, FunctionDeclaration, FunctionType, ImportStatementNode,
    LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode, NamePathNode, NamespaceDeclarationNode, StatementNode,
    StatementType, WhileLoopNode,
};
use valkyrie_types::third_party::pex::{ParseResult, ParseState};

mod classes;
mod lambda;
mod new;
mod normal;

pub struct ReplRoot {
    pub statements: Vec<StatementNode>,
}

pub struct ScriptRoot {
    pub statements: Vec<StatementNode>,
}
