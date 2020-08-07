use crate::{
    helpers::{ignore, parse_name_join},
    ThisParser,
};
use lispify::Lisp;

use valkyrie_ast::{
    ApplyCallNode, ApplyDotNode, CallTermNode, ExpressionBody, GenericArgumentNode, GenericCallNode, IdentifierNode,
    SubscriptNode, SubscriptSliceNode, SubscriptTermNode,
};
use valkyrie_types::third_party::pex::{BracketPair, BracketPattern, ParseResult, ParseState};

mod args_generic;
mod call_apply;
mod call_dot;
mod call_generic;
mod call_index;
