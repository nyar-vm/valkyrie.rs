use crate::{
    helpers::{ignore, parse_name_join},
    utils::get_span,
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPair, BracketPattern, ParseResult, ParseState};
use valkyrie_ast::{
    ApplyArgumentTerm, ApplyCallItem, ApplyCallNode, ApplyDotNode, ArgumentKeyNode, ArgumentTermNode, CallNode, CallTermNode,
    ClosureCallNode, ExpressionNode, GenericArgument, GenericArgumentTerm, GenericCallNode, GenericCallTerm, IdentifierNode,
    LambdaDotNode, LambdaNode, NamePathNode, SubscriptNode, SubscriptSliceNode, SubscriptTermNode, TypingExpression,
};

mod args_apply;
mod args_generic;
mod call_apply;
mod call_dot;
mod call_generic;
mod call_index;
mod call_lambda;
