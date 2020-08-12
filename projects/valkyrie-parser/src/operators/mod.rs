use crate::{traits::ThisParser, utils::get_span};
use lispify::Lisp;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    str::FromStr,
    sync::LazyLock,
};
use valkyrie_ast::{OperatorNode, ValkyrieOperator};
use valkyrie_types::third_party::{
    pex::{
        helpers::{make_from_str, whitespace},
        ParseResult, ParseState, Regex, StopBecause,
    },
    pratt::{Associativity, Precedence},
};

mod display;
mod parser;

#[derive(Clone)]
pub struct ValkyrieInfix {
    normalized: String,
    span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    normalized: String,
    span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    normalized: String,
    span: Range<u32>,
}
