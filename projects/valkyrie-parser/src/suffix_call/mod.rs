use crate::{
    expression::TermExpressionNode,
    helpers::{ignore, parse_name_join},
    ThisParser,
};
use lispify::{Lisp, Lispify};
use std::{
    fmt::{Display, Formatter},
    ops::Range,
};
use valkyrie_ast::{
    ApplyArgumentNode, ApplyCallNode, ApplyDotNode, ApplyTermNode, GenericArgumentNode, GenericCall, IdentifierNode, ViewNode,
    ViewRangeNode, ViewTermNode,
};
use valkyrie_types::third_party::pex::{
    helpers::{make_from_str, whitespace},
    BracketPair, BracketPattern, ParseResult, ParseState, StopBecause,
};

mod call_apply;
mod call_dot;
mod call_index;
