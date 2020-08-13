use crate::{
    helpers::{ignore, parse_name_join},
    utils::get_span,
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPair, BracketPattern, ParseResult, ParseState};
use valkyrie_ast::{
    ApplyCallNode, ApplyCallTerm, ApplyDotNode, ArgumentTermNode, CallNode, CallTermNode, ExpressionBody, ExpressionNode,
    GenericArgumentNode, GenericArgumentTerm, GenericCallNode, IdentifierNode, LambdaCallNode, LambdaDotNode, LambdaNode,
    SubscriptNode, SubscriptSliceNode, SubscriptTermNode,
};

mod args_apply;
mod args_generic;
mod call_apply;
mod call_dot;
mod call_generic;
mod call_index;
mod call_lambda;
