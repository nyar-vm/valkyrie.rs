use crate::{expression::TypeLevelExpressionType, helpers::ignore, looping::FunctionBody, ThisParser};
use lispify::Lisp;
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, ExpressionBody, ExpressionContext, ExpressionNode,
    FunctionDeclarationNode, FunctionType, IdentifierNode, NamePathNode, StatementNode,
};
use valkyrie_types::third_party::pex::{BracketPair, BracketPattern, ParseResult, ParseState, StopBecause};

mod function;
mod methods;
