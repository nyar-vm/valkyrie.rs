use crate::{
    helpers::{ignore, parse_name_join},
    utils::{get_span, parse_expression_body},
    ThisParser,
};
use lispify::Lisp;
use pex::{BracketPair, BracketPattern, ParseResult, ParseState};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyArgumentTerm, ApplyCallNode, ApplyCallTerm, ApplyDotNode, ArgumentKeyNode, ArgumentTermNode,
    CallNode, CallTermNode, ExpressionBody, ExpressionNode, GenericArgumentNode, GenericArgumentTerm, GenericCallNode,
    GenericCallTerm, IdentifierNode, LambdaCallNode, LambdaDotNode, LambdaNode, NamePathNode, SubscriptNode,
    SubscriptSliceNode, SubscriptTermNode, TypingExpression,
};

mod args_apply;
mod args_generic;
mod call_apply;
mod call_dot;
mod call_generic;
mod call_index;
mod call_lambda;
