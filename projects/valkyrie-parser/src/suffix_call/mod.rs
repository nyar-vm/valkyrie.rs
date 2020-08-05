use crate::{
    helpers::{ignore, parse_name_join},
    ThisParser,
};
use lispify::Lisp;

use valkyrie_ast::{
    ApplyCallNode, ApplyDotNode, CallTermPair, ExpressionBody, GenericArgumentNode, GenericCall, IdentifierNode, SubscriptNode,
    SubscriptSliceNode, SubscriptTermNode,
};
use valkyrie_types::third_party::pex::{BracketPair, BracketPattern, ParseResult, ParseState};

mod call_apply;
mod call_dot;
mod call_generic;
mod call_index;
